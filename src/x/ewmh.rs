use x11rb::{atom_manager, protocol::xproto::Atom};

use super::strut::StrutPartialDef;

atom_manager! {
    pub AtomCollection: AtomCollectionCookie {
        _NET_WM_WINDOW_TYPE,
        _NET_WM_WINDOW_TYPE_DESKTOP,
        _NET_WM_WINDOW_TYPE_DIALOG,
        _NET_WM_WINDOW_TYPE_DOCK,
        _NET_WM_WINDOW_TYPE_MENU,
        _NET_WM_WINDOW_TYPE_NOTIFICATION,
        _NET_WM_WINDOW_TYPE_NORMAL,
        _NET_WM_WINDOW_TYPE_SPLASH,
        _NET_WM_WINDOW_TYPE_TOOLBAR,
        _NET_WM_WINDOW_TYPE_UTILITY,
        _NET_WM_STATE,
        _NET_WM_STATE_ABOVE,
        _NET_WM_STATE_BELOW,
        _NET_WM_STATE_DEMANDS_ATTENTION,
        _NET_WM_STATE_FULLSCREEN,
        _NET_WM_STATE_HIDDEN,
        _NET_WM_STATE_MAXIMIZED_VERT,
        _NET_WM_STATE_MAXIMIZED_HORZ,
        _NET_WM_STATE_MODAL,
        _NET_WM_STRUT_PARTIAL,
        _NET_WM_STATE_SHADED,
        _NET_WM_STATE_SKIP_TASKBAR,
        _NET_WM_STATE_SKIP_PAGER,
        _NET_WM_STATE_STICKY,
        _NET_WM_NAME,
        ATOM,
    }
}


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

pub struct EwmhHints {
    pub anchor: Atom,
    pub stack_positiona: Atom,
    pub strut: Option<StrutPartialDef>,
    pub window_type: Atom,
    pub wm_ignore: bool,
}

impl EwmhHints {
    pub fn new(atoms: &AtomCollection, window_type: &WindowType, strut: &Option<StrutPartialDef>) -> Self {
        let window_type = match window_type {
            WindowType::Desktop => atoms._NET_WM_WINDOW_TYPE_DESKTOP,
            WindowType::Dock => atoms._NET_WM_WINDOW_TYPE_DOCK,
            WindowType::Dialog => atoms._NET_WM_WINDOW_TYPE_DIALOG,
            WindowType::Menu => atoms._NET_WM_WINDOW_TYPE_MENU,
            WindowType::Normal => atoms._NET_WM_WINDOW_TYPE_NORMAL,
            WindowType::Notification => atoms._NET_WM_WINDOW_TYPE_NOTIFICATION,
            WindowType::Splash => atoms._NET_WM_WINDOW_TYPE_SPLASH,
            WindowType::Toolbar => atoms._NET_WM_WINDOW_TYPE_TOOLBAR,
            WindowType::Utility => atoms._NET_WM_WINDOW_TYPE_UTILITY,
        };

        let strut = strut.clone();

        Self {
            anchor: 0,
            stack_positiona: 0,
            strut,
            window_type,
            wm_ignore: true,
        }
    }
}
