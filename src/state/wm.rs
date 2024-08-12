use std::fs::File;
use std::sync::RwLock;

use once_cell::sync::Lazy;

pub static WM: Lazy<RwLock<HorizonWm>> = Lazy::new(|| {
    RwLock::new(HorizonWm::new())
});

#[derive(Debug)]
pub struct HorizonWm {
    pub workspaces: Vec<HorizonWmWorkspace>,
    pub scratchpads: Vec<HorizonWmScratchpad>,
}

#[derive(Debug)]
pub struct HorizonWmWorkspace {
    pub name: String,
    pub number: u32,            // INFO: This should be the index of the workspace in a HorizonWm.workspaces.
    pub icon: String,
    pub visible: bool,
    pub screen: Option<u32>,    // INFO: If this workspace is on a screen, the number is the screen index.
    pub layout: String,         // NOTE: Might not need this if the active layout is tracked in layouts.
    pub is_temp: bool,
    pub windows: Vec<HorizonWmWindow>,
    pub layouts: Vec<HorizonWmLayout>
}

#[derive(Debug)]
pub struct HorizonWmWindow {
    pub name: String,
    pub icon: String,
    // pub class: (String, String)  // NOTE: Unsure if we want this. Represents WM_CLASS prop tuple.
    pub focused: bool,
    pub visible: bool,
    pub minimized: bool,
    pub fullscreened: bool,
    pub urgent_flag_set: bool,
}

#[derive(Debug)]
pub struct HorizonWmLayout {
    pub name: String,
    pub icon: String,
    pub active: bool,
}

#[derive(Debug)]
pub struct HorizonWmScratchpad {
    pub name: String,
    pub visible: String,
    pub windows: Vec<HorizonWmWindow>,
}

impl HorizonWm {
    fn new() -> Self {
        todo!()
    }
}

