use std::rc::Rc;

use gdk_x11::prelude::*;
use gdk_x11::{X11Monitor, X11Surface};
use gdk_x11::gdk::Display;
use gtk::prelude::*;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{Atom, AtomEnum, ConnectionExt, PropMode};
use x11rb::rust_connection::RustConnection;

use crate::horizon::{HorizonWindow, WindowType};
use crate::x::ewmh::{AtomCollection, StrutPartialDef};

#[derive(Debug)]
pub struct XSessionContext {
    pub connection: Rc<RustConnection>,
    pub default_display: Display,
    pub monitors: Vec<X11Monitor>,
}

impl XSessionContext {
    pub fn new() -> Self {
        let (connection, screen_idx) = RustConnection::connect(None).unwrap();
        let default_display = Display::default().unwrap();
        let monitors = default_display.monitors()
            .into_iter()
            .map(|x| x.unwrap().downcast::<X11Monitor>().unwrap())
            .collect();

        XSessionContext {
            connection: connection.into(),
            default_display,
            monitors,
        }
    }

    pub fn get_monitor_width(&self, monitor_index: usize) -> i32 {
        if monitor_index > self.monitors.len() {
            panic!("Monitor {} does not exist", monitor_index);
        }

        self.monitors[monitor_index].workarea().width()
    }

    pub fn get_monitor_height(&self, monitor_index: usize) -> i32 {
        if monitor_index > self.monitors.len() {
            panic!("Monitor {} does not exist", monitor_index);
        }

        self.monitors[monitor_index].workarea().height()
    }
}

pub struct XWindowContext {
    xid: Atom,
    atoms: AtomCollection,
    strut: Option<StrutPartialDef>,  // The true strut request accounting for multimonitors
    window_type: u32,
}

impl XWindowContext {
    pub fn new(x_session: Rc<XSessionContext>, horizon_window: &HorizonWindow) -> Self {
        let atoms = AtomCollection::new(&x_session.connection).unwrap()
            .reply().unwrap();

        let xid = horizon_window.window
            .surface()
            .and_downcast_ref::<X11Surface>().expect("Failed to downcast to X11Surface")
            .xid() as Atom;

        let strut = horizon_window.config.strut
            .as_ref()
            .map(|strut| strut.with_offset());

        let window_type = match horizon_window.config.window_type {
            WindowType::Desktop => atoms._NET_WM_WINDOW_TYPE_DESKTOP,
            WindowType::Dock => atoms._NET_WM_WINDOW_TYPE_DOCK,
            WindowType::Dialog => atoms._NET_WM_WINDOW_TYPE_DIALOG,
            WindowType::Normal => atoms._NET_WM_WINDOW_TYPE_NORMAL,
            WindowType::Notification => atoms._NET_WM_WINDOW_TYPE_NOTIFICATION,
            WindowType::Toolbar => atoms._NET_WM_WINDOW_TYPE_TOOLBAR,
            WindowType::Utility => atoms._NET_WM_WINDOW_TYPE_UTILITY,
        };

        XWindowContext {
            xid,
            atoms,
            strut,
            window_type
        }   
    }

    pub fn set_strut_partial_hint(&self, x_session: Rc<XSessionContext>) {
        if self.strut.is_none() {
            return;
        }

        let net_wm_strut_partial = x_session.connection.intern_atom(false, b"_NET_WM_STRUT_PARTIAL")
            .expect("Failed to intern _NET_WM_STRUT_PARTIAL")
            .reply()
            .expect("Failed to get _NET_WM_STRUT_PARTIAL atom");

        let offset_strut = self.strut
            .as_ref()
            .unwrap()
            .as_x11_ready_value();

        x_session.connection.change_property(
            PropMode::REPLACE,
            self.xid,
            self.atoms._NET_WM_STRUT_PARTIAL,
            AtomEnum::CARDINAL,
            32,
            12,
            &offset_strut
        ).expect("Failed to set _NET_WM_STRUT_PARTIAL property").check().unwrap();

        x_session.connection.flush().expect("Failed to flush connection");
        println!("Set _NET_WM_STRUT_PARTIAL");
    }

    pub fn reset_strut_partial_hint(&self, x_session: Rc<XSessionContext>) {
        let zero_strut = StrutPartialDef::get_strut_partial_zero();

        x_session.connection.change_property(
            PropMode::REPLACE,
            self.xid,
            self.atoms._NET_WM_STRUT_PARTIAL,
            AtomEnum::CARDINAL,
            32,
            12,
            &zero_strut.as_x11_ready_value()
        ).expect("Failed to reset _NET_WM_STRUT_PARTIAL property").check().unwrap();

        x_session.connection.flush().expect("Failed to flush connection");
        println!("Reset _NET_WM_STRUT_PARTIAL");
    }

    pub fn set_window_type_hint(&self, x_session: Rc<XSessionContext>) {
        x_session.connection.change_property(
            PropMode::REPLACE,
            self.xid,
            self.atoms._NET_WM_WINDOW_TYPE,
            AtomEnum::ATOM,
            32,
            1,
            &self.window_type.to_le_bytes(),
        ).expect("Failed to set _NET_WM_WINDOW_TYPE property").check().unwrap();

        x_session.connection.flush().expect("Failed to flush connection");
        println!("Set _NET_WM_WINDOW_TYPE");
    }
}
