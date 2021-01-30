use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::{Arc, Mutex},
};

use iced_native::{
    event::Status as EventStatus,
    futures::{
        self,
        channel::{mpsc, oneshot},
        executor::ThreadPool,
        stream::BoxStream,
    },
    Event, Subscription,
};

struct Handle {
    _cancel: oneshot::Sender<()>,
    sender: Option<mpsc::Sender<(Event, EventStatus)>>,
}

#[derive(Default)]
pub struct SubscriptionPool {
    alive: HashMap<u64, Handle>,
}

impl SubscriptionPool {
    pub fn update<Message: Send + 'static>(
        &mut self,
        subscription: Subscription<Message>,
        thread_pool: &mut ThreadPool,
        event_queue: Arc<Mutex<Option<VecDeque<Message>>>>,
    ) {
        use futures::{future::FutureExt, stream::StreamExt};
        let recipes = subscription.recipes();
        let mut alive = HashSet::new();
        for recipe in recipes {
            let hashed = {
                use core::hash::Hasher;
                let mut hasher = iced_native::Hasher::default();
                recipe.hash(&mut hasher);
                hasher.finish()
            };
            alive.insert(hashed);
            if !self.alive.contains_key(&hashed) {
                let (cancel, cancelled) = oneshot::channel();
                let (tx, rx) = mpsc::channel::<(Event, EventStatus)>(100);
                let in_stream: BoxStream<'static, (Event, EventStatus)> = Box::pin(rx);
                let stream: BoxStream<'static, Message> = Box::pin(recipe.stream(in_stream));
                let my_event_queue = event_queue.clone();
                let future = futures::future::select(
                    cancelled,
                    stream.for_each(move |message| {
                        let mut lock = my_event_queue.lock().expect("Poisoned Lock");
                        let mut queue = lock.take().unwrap();
                        queue.push_back(message);
                        *lock = Some(queue);
                        futures::future::ready(())
                    }),
                )
                .map(|_| ());
                thread_pool.spawn_ok(future);

                self.alive.insert(
                    hashed,
                    Handle {
                        _cancel: cancel,
                        sender: if tx.is_closed() { None } else { Some(tx) },
                    },
                );
            }
        }
    }

    pub fn broadcast(&mut self, event: Event) {
        self.alive
            .values_mut()
            .filter_map(|connection| connection.sender.as_mut())
            .for_each(|listener| {
                if let Err(error) = listener.try_send((event.clone(), EventStatus::Captured)) {
                    panic!("Failed to communicate with sender {}", error);
                }
            });
    }
}
