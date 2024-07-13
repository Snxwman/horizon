#![allow(unused_imports)]

use std::rc::Rc;

use gtk::{Application, Window};

use crate::prelude::*;
use crate::x::ewmh::StrutPartialDef;
use crate::x::x::XSessionContext;

/// [Convenience for user config] default display number is usually 0.
const MONITOR: usize = 0;
/// [Convenience for user config] default is meaningless.
const HEIGHT: i32 = 30;

/// Possible types for `_NET_WM_WINDOW_TYPE`.
#[derive(Debug)]
pub enum WindowType {
    Desktop,
    Dialog,
    Dock,
    Menu,
    Notification,  // NOTE: Does not appear in the ewmh spec
    Normal,
    Splash,
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
    pub width: i32,
    pub height: i32,
}

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

/// User defined window configuration.
#[derive(Debug)]
pub struct HorizonWindowConfig {
    /// The X display number of the screen the window should be drawn on.
    pub screen: usize,
    /// The desired width and height of the window in pixels.
    pub size: Size,
    /// A coordinate denoting where on the screen the window should be placed.
    pub position: Position,
    // TODO:
    pub anchor: WindowAnchor,
    /// Whether the window manager should ignore this window.
    ///
    /// Setting this to `true` sets the following hints for the `_NET_WM_STATE` property:
    ///
    /// - `_NET_WM_STATE_SKIP_PAGER` (Prevents the window from showing up in pagers)
    /// - `_NET_WM_STATE_SKIP_TASKBAR` (Prevents the window from showing up in taskbars)
    pub wm_ignore: bool,
    // TODO:
    pub stack_position: WindowStackPosition,
    /// The window type to set for _NET_WM_WINDOW_TYPE.
    pub window_type: WindowType,
    /// A reserved space on the screen where no other windows will overlap.
    pub strut: Option<StrutPartialDef>,
}

pub struct HorizonWindow {
    pub name: String,
    pub config: HorizonWindowConfig,
    pub gtk_window: Window,
}

pub fn get_windows(horizon: &Application, x_session: Rc<XSessionContext>) -> Vec<HorizonWindow> {
    let strut = StrutPartialDef::builder()
        .xsession(x_session.clone())
        .monitor(MONITOR)
        .size(HEIGHT as u32 + 5)
        .top()
        .full_length(Side::Top)
        .build();

    let config = HorizonWindowConfig {
        screen: MONITOR,
        size: Size {width: x_session.get_monitor_width(MONITOR), height: HEIGHT},
        position: Position {x: 0, y: 45},
        anchor: WindowAnchor::TopLeft,
        wm_ignore: true,
        stack_position: WindowStackPosition::Background,
        window_type: WindowType::Dock,
        strut: Some(strut),
    };

    let clock = Clock::new();

    let gtk_window = Window::builder()
        .application(horizon)
        .default_width(config.size.width)
        .default_height(config.size.height)
        .resizable(false)
        .focusable(true)
        .focus_on_click(true)
        .child(&clock.widget())
        .build();

    vec![
        HorizonWindow {name: String::from("fullbar"), config, gtk_window}
    ]
}
