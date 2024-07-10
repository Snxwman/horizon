use std::fs::File;
use std::sync::RwLock;

use once_cell::sync::Lazy;

pub static WM: Lazy<RwLock<HorizonWm>> = Lazy::new(|| {
    RwLock::new(HorizonWm::new())
});

#[derive(Debug)]
pub struct HorizonWm {}

impl HorizonWm {
    fn new() -> Self {
        todo!()
    }
}

