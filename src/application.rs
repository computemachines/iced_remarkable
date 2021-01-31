use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use iced_core::Point;
use iced_graphics::Renderer;
use iced_native::{futures, Cache, Command, Container, Element, Subscription, UserInterface};

use crate::{
    backend::{RemarkableBackend, DISPLAYHEIGHT, DISPLAYWIDTH},
    subscription_pool::SubscriptionPool,
    RemarkableRenderer,
};

pub trait Application: Sized {
    type Message: std::fmt::Debug + Send + Sync + Clone;

    fn new() -> (Self, Command<Self::Message>);

    fn update(&mut self, messages: Vec<Self::Message>) -> Vec<Command<Self::Message>>;

    fn view(&mut self) -> Element<'_, Self::Message, RemarkableRenderer>;

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    fn run()
    where
        Self: 'static,
    {
        let mut renderer: RemarkableRenderer = Renderer::new(RemarkableBackend::new());
        let (mut state, command) = Self::new();
        let mut cache = Some(Cache::default());

        let mut thread_pool = futures::executor::ThreadPool::new().unwrap();
        let mut subscription_pool = SubscriptionPool::default();

        let event_queue = Arc::new(Mutex::new(Some(VecDeque::default())));

        spawn_command(command, &mut thread_pool, event_queue.clone());

        const SIZE: [u16; 2] = [DISPLAYWIDTH, DISPLAYHEIGHT];

        let mut count = 0;

        loop {
            subscription_pool.update(state.subscription(), &mut thread_pool, event_queue.clone());

            let state_view = state.view();

            let view: Element<'_, Self::Message, RemarkableRenderer> = Container::new(state_view)
                .width(SIZE[0].into())
                .height(SIZE[1].into())
                .into();

            let mut ui =
                UserInterface::build(view, SIZE.into(), cache.take().unwrap(), &mut renderer);

            let primitives = ui.draw(&mut renderer, Point::ORIGIN);
            dbg!(&primitives);
            renderer.backend_mut().clear();
            renderer.backend_mut().render(&primitives.0);
            renderer.backend_mut().update_full();

            let mut messages = vec![];
            let mut evt_queue = event_queue.lock().expect("Poisoned lock");
            let mut events = evt_queue.take().unwrap();
            messages.append(&mut events.drain(..).collect());
            *evt_queue = Some(VecDeque::default());
            drop(evt_queue);

            cache = Some(ui.into_cache());

            thread::sleep(Duration::from_millis(100000));
            count += 1;
            if count >= 1 {
                break;
            }
        }
    }
}

fn spawn_command<Message: Send + 'static>(
    command: Command<Message>,
    thread_pool: &mut futures::executor::ThreadPool,
    event_queue: Arc<Mutex<Option<VecDeque<Message>>>>,
) {
    use iced_native::futures::FutureExt;
    let futures = command.futures();
    for future in futures {
        let event_queue = event_queue.clone();
        let future = future.map(move |message| {
            let mut my_event_queue = event_queue.lock().unwrap();
            let mut taken = my_event_queue.take().unwrap();
            taken.push_back(message);
            *my_event_queue = Some(taken);
        });
        thread_pool.spawn_ok(future);
    }
}
