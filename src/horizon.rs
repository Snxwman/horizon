use std::rc::Rc;

use gtk::{Application, Window};

use crate::util::Side; 
use crate::x::ewmh::StrutPartialDef;
use crate::x::x::XSessionContext;

const MONITOR: usize = 0;
const HEIGHT: i32 = 30;

#[derive(Debug)]
pub enum WindowType {
    Desktop,
    Dialog,
    Dock,
    Notification,
    Normal,
    Toolbar,
    Utility,
}

#[derive(Debug)]
pub enum WindowAnchor {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    CenterCenter,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

#[derive(Debug)]
pub enum WindowStackPosition {
    Foreground,
    Background,
}

#[derive(Debug)]
pub struct Size {
    pub height: i32,
    pub width: i32,
}

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct HorizonWindowConfig {
    pub screen: usize,
    pub size: Size,
    pub position: Position, 
    pub anchor: WindowAnchor,
    pub wm_ignore: bool,
    pub stack_position: WindowStackPosition,
    pub window_type: WindowType,
    pub strut: Option<StrutPartialDef>,
}

pub struct HorizonWindow {
    pub name: String,
    pub config: HorizonWindowConfig,
    pub window: gtk::Window,
}

pub fn get_windows(horizon: &Application, x_session: Rc<XSessionContext>) -> Vec<HorizonWindow>{
    let strut = StrutPartialDef::builder()
            .full_length(Side::Top, HEIGHT as u32)
            .build();

    let config = HorizonWindowConfig {
        screen: MONITOR,
        size: Size {height: HEIGHT, width: x_session.get_monitor_width(MONITOR)},
        position: Position {x: 0, y: 0},
        anchor: WindowAnchor::TopLeft,
        wm_ignore: true,
        stack_position: WindowStackPosition::Background,
        window_type: WindowType::Dock,
        strut: Some(strut),
    };

    let window = Window::builder()
        .application(horizon)
        .default_width(config.size.width)
        .default_height(config.size.height)
        .resizable(false)
        // .child(child)
        .build();

    vec![
        HorizonWindow {name: String::from("fullbar"), config, window}
    ]
}
