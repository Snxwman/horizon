#![allow(unused_imports)]

mod datetime;
mod wm;

pub use crate::state::{
    datetime::DATETIME,
    datetime::HorizonDateTime,
    wm::WM,
    wm::HorizonWm,
};

pub trait HorizonState {
    
}

#[derive(Debug)]
pub enum ChannelMessage {
    Init,
    Updated,
}

#[derive(Debug)]
pub enum Provider {
    Function,
    JsonFile(String),
    External(String),
    Dbus(String),
}
