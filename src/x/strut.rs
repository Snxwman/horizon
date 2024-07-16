use std::{fmt, rc::Rc};

use crate::util::*;
use crate::x::x::XSessionContext;

#[derive(Debug, Clone, Default)]
pub struct StrutPartialDef {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
    left_start_y: i32,
    left_end_y: i32,
    right_start_y: i32,
    right_end_y: i32,
    top_start_x: i32,
    top_end_x: i32,
    bottom_start_x: i32,
    bottom_end_x: i32,
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

    fn as_vec(&self) -> Vec<i32> {
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
    size: i32,
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
    left_start_y: i32,
    left_end_y: i32,
    right_start_y: i32,
    right_end_y: i32,
    top_start_x: i32,
    top_end_x: i32,
    bottom_start_x: i32,
    bottom_end_x: i32,
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
        // if !self.non_zero_value_set {
        //     return StrutPartialDef::default();
        // }

        // Unable to calculate offsets without an XSessionContext
        // match self.x_session.is_none() && self.should_apply_offsets {
        //     true => panic!("No XSessionContext provided, so offsets can't be applied."),
        //     false => (),
        // }

        // Specifying strut coordinates without a size implies a mistake.
        // Zero struts should be all zero.
        // match self.non_zero_value_set && self.size == 0 {
        //     true => panic!("A non-zero strut was specified, but no size was specified."),
        //     false => (),
        // }

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

    pub fn strut_width(&self, size: Number) -> i32 {
        match size {
            Number::Absolute(h) => h,
            Number::Percent(p) => {
                let monitor_height = self.x_session
                    .as_ref()
                    .unwrap()
                    .get_monitor_height(self.monitor);

                monitor_height * p
            }
        }
    }

    pub fn strut_length(&self, size: Number) -> (i32, i32) {
        todo!()
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

    // TODO: Support percentages
    pub fn size(mut self, size: i32) -> Self {
        self.size = size;
        self
    }

    pub fn top(mut self, size: Number) -> Self {
        self.top = self.strut_width(size);
        self
    }

    pub fn bottom(mut self, size: Number) -> Self {
        self.bottom = self.strut_width(size);
        self
    }

    pub fn left(mut self, size: Number) -> Self {
        self.left = self.strut_width(size);
        self
    }

    pub fn right(mut self, size: Number) -> Self {
        self.right = self.strut_width(size);
        self
    }

    pub fn full_length(mut self, side: Side) -> Self {
        let monitor_width = self.x_session
            .as_ref()
            .unwrap()
            .get_monitor_width(self.monitor);
        let monitor_height = self.x_session
            .as_ref()
            .unwrap()
            .get_monitor_height(self.monitor);

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

    pub fn partial_length(mut self, start: i32, end: i32) -> Self {
        self.top_start_x = start;
        self.top_end_x = end;
        self
    }
}

struct StrutPartialDefBuilderIntSize {
    side: Side,
    size: i32,
    length: (i32, i32),
    prev_builder: StrutPartialDefBuilder,
}

impl StrutPartialDefBuilderIntSize {
    fn new(prev_builder: StrutPartialDefBuilder, side: Side, size: i32) -> Self {
        Self {
            side,
            size,
            length: (0, 0),
            prev_builder,
        }
    }

    fn full_length(mut self) -> StrutPartialDefBuilder {
        todo!()
    }

    fn partial_length(mut self) -> StrutPartialDefBuilder {
        todo!()
    }
}

struct StrutPartialDefBuilderIntLength {
    side: Option<Side>,
    size: Option<i32>,
    length: (i32, i32),
    prev_builder: StrutPartialDefBuilder,
}

impl StrutPartialDefBuilderIntLength {
    fn new(prev_builder: StrutPartialDefBuilder) -> Self {
        todo!()
    }


}
