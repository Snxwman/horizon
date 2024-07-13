use std::{fmt, rc::Rc, u32};

use x11rb::atom_manager;

use crate::util::Side;

use super::x::XSessionContext;

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

#[derive(Debug, Clone, Default)]
pub struct StrutPartialDef {
    left: u32,
    right: u32,
    top: u32,
    bottom: u32,
    left_start_y: u32,
    left_end_y: u32,
    right_start_y: u32,
    right_end_y: u32,
    top_start_x: u32,
    top_end_x: u32,
    bottom_start_x: u32,
    bottom_end_x: u32,
}

impl StrutPartialDef {
    pub fn builder() -> StrutPartialDefBuilder {
        StrutPartialDefBuilder::new()
    }

    pub fn as_x11_ready_value(&self) -> Vec<u8> {
        self.as_vec()
            .iter()
            .flat_map(|x| x.to_le_bytes().to_vec())
            .collect()
    }

    fn as_vec(&self) -> Vec<u32> {
        vec![
            self.left,
            self.right,
            self.top,
            self.bottom,
            self.left_start_y,
            self.left_end_y,
            self.right_start_y,
            self.right_end_y,
            self.top_start_x,
            self.top_end_x,
            self.bottom_start_x,
            self.bottom_end_x
        ]
    }

}

impl fmt::Display for StrutPartialDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
            self.left,
            self.right,
            self.top,
            self.bottom,
            self.left_start_y,
            self.left_end_y,
            self.right_start_y,
            self.right_end_y,
            self.top_start_x,
            self.top_end_x,
            self.bottom_start_x,
            self.bottom_end_x,
        )
    }
}

#[derive(Clone, Default)]
pub struct StrutPartialDefBuilder {
    x_session: Option<Rc<XSessionContext>>,
    monitor: usize,
    should_apply_offsets: bool,
    // TODO: Account for existing struts when making new ones.
    existing_struts: Option<Vec<StrutPartialDef>>,
    // Does not track if size is set. That is handles by checking if size is 0.
    non_zero_value_set: bool,
    size: u32,
    left: u32,
    right: u32,
    top: u32,
    bottom: u32,
    left_start_y: u32,
    left_end_y: u32,
    right_start_y: u32,
    right_end_y: u32,
    top_start_x: u32,
    top_end_x: u32,
    bottom_start_x: u32,
    bottom_end_x: u32,
}

impl StrutPartialDefBuilder {
    fn new() -> Self {
        Self {
            x_session: None,
            should_apply_offsets: true,
            existing_struts: None,
            ..Default::default()
        }
    }

    pub fn build(mut self) -> StrutPartialDef {
        // No strut coordinates implies a zero strut.
        if !self.non_zero_value_set {
            return StrutPartialDef::default();
        }

        // Unable to calculate offsets without an XSessionContext
        match self.x_session.is_none() && self.should_apply_offsets {
            true => panic!("No XSessionContext provided, so offsets can't be applied."),
            false => (),
        }

        // Specifying strut coordinates without a size implies a mistake.
        // Zero struts should be all zero.
        match self.non_zero_value_set && self.size == 0 {
            true => panic!("A non-zero strut was specified, but no size was specified."),
            false => (),
        }

        if self.should_apply_offsets {
            self.apply_offsets();
        }

        StrutPartialDef {
            left: self.left,
            right: self.right,
            top: self.top,
            bottom: self.bottom,
            left_start_y: self.left_start_y,
            left_end_y: self.left_end_y,
            right_start_y: self.right_start_y,
            right_end_y: self.right_end_y,
            top_start_x: self.top_start_x,
            top_end_x: self.top_end_x,
            bottom_start_x: self.bottom_start_x,
            bottom_end_x: self.bottom_end_x,
        }
    }

    fn apply_offsets(&mut self) {
        let x_session = self.x_session.as_ref().unwrap();
        let (monitor_start_x, monitor_end_x, monitor_start_y, monitor_end_y) = x_session.get_monitor_bounds(self.monitor);
        let (display_width, display_height) = x_session.display_bounds;

        self.left += monitor_start_x;
        self.right += display_width - monitor_end_x;
        self.top += monitor_start_y;
        self.bottom += display_height - monitor_end_y;
    }

    pub fn xsession(mut self, x_session: Rc<XSessionContext>) -> Self {
        self.x_session = Some(x_session);
        self
    }

    pub fn monitor(mut self, monitor: usize) -> Self {
        self.monitor = monitor;
        self
    }

    pub fn do_apply_offsets(mut self) -> Self {
        self.should_apply_offsets = true;
        self
    }

    pub fn dont_apply_offsets(mut self) -> Self {
        self.should_apply_offsets = false;
        self
    }

    pub fn single_monitor(self) -> Self {
        self.dont_apply_offsets()
    }

    // TODO: Support percentages
    pub fn size(mut self, size: u32) -> Self {
        self.size = size;
        self
    }

    pub fn top(mut self) -> Self {
        self.top = self.size;
        self
        // let end_x = match end_x {
        //     x if x >= 0 => end_x,
        //     x if x < 0 => 5120 + end_x,
        //     _ => unreachable!(),
        // };
        //
        // self.top = self.size;
        // self.top_start_x = start_x;
        // self.top_end_x = end_x as u32;
        //
        // self.non_zero_value_set = true;
        // self
    }

    pub fn bottom(mut self) -> Self {
        todo!()
    }

    pub fn left(mut self) -> Self {
        todo!()
    }

    pub fn right(mut self) -> Self {
        todo!()
    }

    pub fn full_length(mut self, side: Side) -> Self {
        let monitor_width = self.x_session
            .as_ref()
            .unwrap()
            .get_monitor_width(self.monitor) as u32;
        let monitor_height = self.x_session
            .as_ref()
            .unwrap()
            .get_monitor_height(self.monitor) as u32;

        match side {
            Side::Left => {
                self.left = self.size;
                self.left_end_y = monitor_height;
            },
            Side::Right => {
                self.right = self.size;
                self.right_end_y = monitor_height;
            },
            Side::Top => {
                self.top = self.size;
                self.top_end_x = monitor_width;
            },
            Side::Bottom => {
                self.bottom = self.size;
                self.bottom_end_x = monitor_width;
            }
        }

        self.non_zero_value_set = true;
        self
    }
}

