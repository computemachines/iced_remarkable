use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use iced_core::Point;
use iced_graphics::Renderer;
use iced_native::{
    futures, mouse, Cache, Command, Container, Element, Event, Subscription, UserInterface,
};
use libremarkable::input::{multitouch::Finger, InputEvent};
use log;
use logging_timer::{stime, time};

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
        use simple_logger::SimpleLogger;
        SimpleLogger::new().init().unwrap();
        log::info!("application running");

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
            // dbg!(&primitives);
            renderer.backend_mut().clear();

            renderer.backend_mut().render(&primitives.0);
            renderer.backend_mut().update_full();

            let events = renderer
                .backend_mut()
                .input_rx
                .try_iter()
                .map(|e| match e {
                    InputEvent::WacomEvent { event } => unimplemented!("WacomEvent"),
                    InputEvent::MultitouchEvent { event } => (
                        Event::Mouse(match event {
                            libremarkable::input::multitouch::MultitouchEvent::Press { finger } => {
                                mouse::Event::ButtonPressed(mouse::Button::Left)
                            }
                            libremarkable::input::multitouch::MultitouchEvent::Release {
                                finger,
                            } => mouse::Event::ButtonReleased(mouse::Button::Left),
                            libremarkable::input::multitouch::MultitouchEvent::Move { finger } => {
                                mouse::Event::CursorMoved {
                                    x: finger.pos.x as f32,
                                    y: finger.pos.y as f32,
                                }
                            }
                            libremarkable::input::multitouch::MultitouchEvent::Unknown => {
                                panic!("Unknown MultitouchEvent")
                            }
                        }),
                        event.finger().map(|f| f.clone()),
                    ),
                    InputEvent::GPIO { event } => unimplemented!("GPIO"),
                    InputEvent::Unknown {} => panic!("Unknown InputEvent"),
                })
                .collect::<Vec<(Event, Option<Finger>)>>();
            let mut messages = vec![];
            for (event, finger) in events {
                let single_event = [event];
                let cursor_position = finger.map_or(Point::new(0.0, 0.0), |finger| {
                    Point::new(finger.pos.x as f32, finger.pos.y as f32)
                });
                ui.update(
                    &single_event,
                    cursor_position,
                    None,
                    &renderer,
                    &mut messages,
                );
            }
            dbg!(&messages);
            let mut evt_queue = event_queue.lock().expect("Poisoned lock");
            let mut events = evt_queue.take().unwrap();
            messages.append(&mut events.drain(..).collect());
            *evt_queue = Some(VecDeque::default());
            drop(evt_queue);

            cache = Some(ui.into_cache());

            if messages.len() != 0 {
                let commands = state.update(messages);
                for command in commands {
                    spawn_command(command, &mut thread_pool, event_queue.clone());
                }
            }

            thread::sleep(Duration::from_millis(1000));
            count += 1;
            if count >= 10 {
                break;
            }
        }
    }
}

fn spawn_command<Message: Send + std::fmt::Debug + 'static>(
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
            log::info!("Pushing message: {:?}", message);
            taken.push_back(message);
            *my_event_queue = Some(taken);
        });
        thread_pool.spawn_ok(future);
    }
}
