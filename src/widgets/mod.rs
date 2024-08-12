#![allow(unused_imports)]

mod clock;
mod icon;
mod workspaces;

pub use crate::widgets::{
    clock::Clock,
    icon::Icon,
    workspaces::Workspaces,
};

pub trait HorizonWidget {
    fn update();
}
