use std::fs::File;
use std::sync::RwLock;

use once_cell::sync::Lazy;

pub static DATETIME: Lazy<RwLock<Wm>> = Lazy::new(|| {
    RwLock::new(Wm::new())
});

#[derive(Debug)]
pub struct Wm {}

impl Wm {
    fn new() -> Self {
        todo!()
    }
}

