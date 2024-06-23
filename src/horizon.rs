use std::rc::Rc;

use gdk_x11::X11Surface;
use gtk::prelude::*;
use gtk::ApplicationWindow;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{Atom, AtomEnum, ConnectionExt, PropMode};
use x11rb::rust_connection::RustConnection;

use crate::ewmh::StrutPartialDef;

x11rb::atom_manager! {
    pub AtomCollection: AtomCollectionCookie {
        _NET_WM_WINDOW_TYPE,
        _NET_WM_WINDOW_TYPE_DESKTOP,
        _NET_WM_WINDOW_TYPE_DIALOG,
        _NET_WM_WINDOW_TYPE_DOCK,
        _NET_WM_WINDOW_TYPE_NOTIFICATION,
        _NET_WM_WINDOW_TYPE_NORMAL,
        _NET_WM_WINDOW_TYPE_TOOLBAR,
        _NET_WM_WINDOW_TYPE_UTILITY,
        _NET_WM_STATE,
        _NET_WM_STATE_ABOVE,
        _NET_WM_STATE_BELOW,
        _NET_WM_STATE_STICKY,
        _NET_WM_STRUT_PARTIAL,
        ATOM,
    }
}

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
enum WindowAnchor {
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
enum WindowStackPosition {
    Foreground,
    Background,
}

#[derive(Debug)]
pub struct HorizonWindowConfig {
    screen: u8,
    size: (u32, u32),
    position: (u32, u32),
    anchor: WindowAnchor,
    wm_ignore: bool,
    stack_position: WindowStackPosition,
    window_type: WindowType,
}

#[derive(Debug, Clone)]
pub struct HorizonXSurfaceContext {
    connection: Rc<RustConnection>,
    root: u32,
    atoms: AtomCollection,
    xid: Atom,
    strut: Option<StrutPartialDef>,  // Maybe should be in window config?
    offset_strut: Option<StrutPartialDef>,  // Actual strut location accounting for multimonitor
}

impl HorizonXSurfaceContext {
    pub fn new(gtk_app_window: &ApplicationWindow, strut: Option<StrutPartialDef>) -> Self {
        let (connection, screen_num) = RustConnection::connect(None)
            .expect("Failed to connect to X11 server");

        let root = connection.setup().roots[screen_num].root;
        let atoms = AtomCollection::new(&connection).unwrap().reply().unwrap();

        let xid = gtk_app_window
            .surface()
            .and_downcast_ref::<X11Surface>().expect("Failed to downcast to X11Surface")
            .xid() as Atom;

        let offset_strut = if strut.is_some() {
            Some(strut.clone().unwrap().with_offset())
        } else {
            None
        };

        HorizonXSurfaceContext {
            connection: connection.into(),
            root,
            atoms,
            xid,
            strut,
            offset_strut, 
        }
    }

    pub fn set_strut_partial(&self) {
        if self.offset_strut.is_none() {
            return;
        }

        let net_wm_strut_partial = self.connection.intern_atom(false, b"_NET_WM_STRUT_PARTIAL")
            .expect("Failed to intern _NET_WM_STRUT_PARTIAL")
            .reply()
            .expect("Failed to get _NET_WM_STRUT_PARTIAL atom");

        let offset_strut = self.offset_strut
            .as_ref()
            .unwrap()
            .as_x11_ready_value();

        self.connection.change_property(
            PropMode::REPLACE,
            self.xid,
            self.atoms._NET_WM_STRUT_PARTIAL,
            AtomEnum::CARDINAL,
            32,
            12,
            &offset_strut
        ).expect("Failed to set _NET_WM_STRUT_PARTIAL property").check().unwrap();

        self.connection.flush().expect("Failed to flush connection");
        println!("Set _NET_WM_STRUT_PARTIAL");
    }

    pub fn reset_strut_partial(&self) {
        let zero_strut = StrutPartialDef::get_strut_partial_zero();

        self.connection.change_property(
            PropMode::REPLACE,
            self.xid,
            self.atoms._NET_WM_STRUT_PARTIAL,
            AtomEnum::CARDINAL,
            32,
            12,
            &zero_strut.as_x11_ready_value()
        ).expect("Failed to reset _NET_WM_STRUT_PARTIAL property").check().unwrap();

        self.connection.flush().expect("Failed to flush connection");
        println!("Reset _NET_WM_STRUT_PARTIAL");
    }

    pub fn set_window_type(&self) {
        // TODO: get this value from window config
        let window_type_data = self.atoms._NET_WM_WINDOW_TYPE_DOCK.to_le_bytes().to_vec();

        self.connection.change_property(
            PropMode::REPLACE,
            self.xid,
            self.atoms._NET_WM_WINDOW_TYPE,
            AtomEnum::ATOM,
            32,
            1,
            &window_type_data,
        ).expect("Failed to set _NET_WM_WINDOW_TYPE property").check().unwrap();

        self.connection.flush().expect("Failed to flush connection");
        println!("Set _NET_WM_WINDOW_TYPE");
    }
}

