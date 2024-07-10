use std::collections::HashMap;
use std::sync::RwLock;

use async_channel::Sender;
use once_cell::sync::Lazy;

pub static EVENT_MANAGER: Lazy<EventManager> = Lazy::new(|| {
    EventManager::new()
});

#[derive(Debug, Clone)]
pub enum Event {
    HorizonDateTimeUpdated,
    HorizonWmUpdated,
}

pub struct EventManager {
    listeners: RwLock<HashMap<usize, Sender<Event>>>,
}

impl EventManager {
    pub fn new() -> Self {
        let listeners = RwLock::new(HashMap::new());

        Self {
            listeners
        }
    }

    pub fn register_listener(&self, listener: Sender<Event>) -> usize {
        todo!()
    }

    pub fn unregister_listener(&self, index: &usize) {
        todo!()
    }

    pub fn notify_listeners(&self, event: Event) {
        todo!()
    }
}
