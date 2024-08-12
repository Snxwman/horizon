use std::cmp::max;
use std::rc::Rc;

use gdk_x11::prelude::*;
use gdk_x11::{X11Monitor, X11Surface};
use gdk_x11::gdk::Display;
use gtk::prelude::*;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{Atom, AtomEnum, ConfigureWindowAux, ConnectionExt, PropMode};
use x11rb::rust_connection::RustConnection;

use crate::horizon::{HorizonWindow, HorizonWindowConfig};
use crate::x::ewmh::{AtomCollection, WindowType, EwmhHints};
use crate::x::strut::StrutPartialDef; 

#[derive(Debug)]
pub struct XSessionContext {
    pub connection: Rc<RustConnection>,
    pub display: Display,
    pub display_bounds: (i32, i32),
    pub monitors: Vec<X11Monitor>,
}

impl XSessionContext {
    pub fn new() -> Self {
        let (connection, screen_idx) = RustConnection::connect(None).unwrap();
        let display = Display::default().unwrap();
        let monitors: Vec<_> = display.monitors()
            .into_iter()
            .map(|x| x.unwrap().downcast::<X11Monitor>().unwrap())
            .collect();

        let mut display_width = 0;
        let mut display_height = 0;

        for monitor in &monitors {
            let end_x = monitor.workarea().x() + monitor.workarea().width();
            let end_y = monitor.workarea().y() + monitor.workarea().height();
            display_width = max(display_width, end_x);
            display_height = max(display_height, end_y);
        }

        Self {
            connection: connection.into(),
            display,
            display_bounds: (display_width, display_height),
            monitors,
        }
    }

    pub fn get_monitor_offsets(&self, monitor_index: usize) -> (i32, i32) {
        let workarea = self.monitors[monitor_index].workarea();
        (workarea.x(), workarea.y())
    }

    pub fn get_monitor_bounds(&self, monitor_index: usize) -> (i32, i32, i32, i32) {
        let workarea = self.monitors[monitor_index].workarea();

        // (start_x, end_x, start_y, end_y)
        (
            workarea.x(), workarea.x() + workarea.width(),
            workarea.y(), workarea.y() + workarea.height()
        )
    }

    pub fn get_monitor_width(&self, monitor_index: usize) -> i32 {
        self.monitors[monitor_index].workarea().width()
    }

    pub fn get_monitor_height(&self, monitor_index: usize) -> i32 {
        self.monitors[monitor_index].workarea().height()
    }
}

pub struct XWindowContext {
    surface: X11Surface,
    xid: Atom,
    atoms: AtomCollection,
    ewmh: EwmhHints,
}

impl XWindowContext {
    pub fn new(x_session: Rc<XSessionContext>, horizon_window: &HorizonWindow) -> Self {
        let surface = horizon_window.gtk_window
            .surface()
            .and_downcast::<X11Surface>()
            .expect("Failed to downcast to X11Surface");

        let xid = surface.xid() as Atom;

        let atoms = AtomCollection::new(&x_session.connection).unwrap()
            .reply().unwrap();

        let ewmh = EwmhHints::new(&atoms, &horizon_window.config.window_type, &horizon_window.config.strut);

        Self {
            surface,
            xid,
            atoms,
            ewmh,
        }
    }


    pub fn configure_xwindow(&self, x_session: Rc<XSessionContext>, horizon_window: &HorizonWindow) {
        self.set_ewmh_hints(x_session.clone(), horizon_window);
        self.move_window(x_session.clone(), &horizon_window.config);
    }

    pub fn move_window(&self, x_session: Rc<XSessionContext>, horizon_window: &HorizonWindowConfig) {
        let (monitor_start_x, monitor_start_y) = x_session.get_monitor_offsets(horizon_window.screen);

        let window_config = ConfigureWindowAux::new()
            .x(monitor_start_x as i32 + horizon_window.position.x)
            .y(monitor_start_y as i32 + horizon_window.position.y);
            // .width(horizon_window.size.width as u32)
            // .height(horizon_window.size.height);

        let _ = x_session.connection.configure_window(self.xid, &window_config).unwrap();
        x_session.connection.flush().expect("Failed to configure X window.");
        dbg!("{:?}", window_config);
    }

    pub fn set_ewmh_hints(&self, x_session: Rc<XSessionContext>, horizon_window: &HorizonWindow) {
        self.set_window_type_hint(x_session.clone());
        self.set_strut_partial_hint(x_session.clone());

        // NOTE: Eventually will create our own methods like set_strut_partial_hint() rather than use gtk's builtins.
        // Both for consistency and to support setting/resetting the other _NET_WM_STATE_* hints.
        self.surface.set_skip_taskbar_hint(horizon_window.config.wm_ignore);
        self.surface.set_skip_pager_hint(horizon_window.config.wm_ignore);
    }

    fn set_strut_partial_hint(&self, x_session: Rc<XSessionContext>) {
        if self.ewmh.strut.is_none() {
            return;
        }

        let net_wm_strut_partial = x_session.connection.intern_atom(false, b"_NET_WM_STRUT_PARTIAL")
            .expect("Failed to intern _NET_WM_STRUT_PARTIAL")
            .reply()
            .expect("Failed to get _NET_WM_STRUT_PARTIAL atom");

        let strut = self.ewmh.strut.as_ref().unwrap().as_x11_ready_value();

        x_session.connection.change_property(
            PropMode::REPLACE,
            self.xid,
            self.atoms._NET_WM_STRUT_PARTIAL,
            AtomEnum::CARDINAL,
            32,
            12,
            &strut
        ).expect("Failed to set _NET_WM_STRUT_PARTIAL property").check().unwrap();

        x_session.connection.flush().expect("Failed to flush connection");
        println!("Set _NET_WM_STRUT_PARTIAL");
    }

    fn reset_strut_partial_hint(&self, x_session: Rc<XSessionContext>) {
        let zero_strut = StrutPartialDef::default().as_x11_ready_value();

        // NOTE: use delete_property()
        x_session.connection.change_property(
            PropMode::REPLACE,
            self.xid,
            self.atoms._NET_WM_STRUT_PARTIAL,
            AtomEnum::CARDINAL,
            32,
            12,
            &zero_strut
        ).expect("Failed to reset _NET_WM_STRUT_PARTIAL property").check().unwrap();

        x_session.connection.flush().expect("Failed to flush connection");
        // println!("Reset _NET_WM_STRUT_PARTIAL");
    }

    fn set_window_type_hint(&self, x_session: Rc<XSessionContext>) {
        x_session.connection.change_property(
            PropMode::REPLACE,
            self.xid,
            self.atoms._NET_WM_WINDOW_TYPE,
            AtomEnum::ATOM,
            32,
            1,
            &self.ewmh.window_type.to_le_bytes(),
        ).expect("Failed to set _NET_WM_WINDOW_TYPE property").check().unwrap();

        x_session.connection.flush().expect("Failed to flush connection");
        // println!("Set _NET_WM_WINDOW_TYPE");
    }
}
