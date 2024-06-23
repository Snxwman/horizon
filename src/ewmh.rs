use std::fmt;

use crate::util::Side;

#[derive(Debug, Clone)]
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

    pub fn with_offset(&self) -> StrutPartialDef {
        StrutPartialDef {
            top: self.top + 1080, 
            ..self.clone()
        }
    }

    pub fn get_strut_partial_zero() -> Self {
        StrutPartialDef {
            left: 0,
            right: 0,
            top: 0,
            bottom: 0,
            left_start_y: 0,
            left_end_y: 0,
            right_start_y: 0,
            right_end_y: 0,
            top_start_x: 0,
            top_end_x: 0,
            bottom_start_x: 0,
            bottom_end_x: 0,
        }
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

    pub fn as_x11_ready_value(&self) -> Vec<u8> {
        self.as_vec()
            .iter()
            .flat_map(|x| x.to_le_bytes().to_vec())
            .collect()
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
            self.bottom_end_x
        )
    }
}

pub struct StrutPartialDefBuilder {
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
         StrutPartialDefBuilder {
            left: 0,
            right: 0,
            top: 0,
            bottom: 0,
            left_start_y: 0,
            left_end_y: 0,
            right_start_y: 0,
            right_end_y: 0,
            top_start_x: 0,
            top_end_x: 0,
            bottom_start_x: 0,
            bottom_end_x: 0,
        }
    }

    pub fn build(self) -> StrutPartialDef {
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

    pub fn full_length(mut self, side: Side, size: u32) -> Self {
        // TODO: get these dynamically (only set for testing)
        let width: u32 = 5120 - 1;
        let height: u32 = 1440 - 1;

        match side {
            Side::Left => {
                self.left = size;
                self.left_end_y = height;
            },
            Side::Right => {
                self.right = size;
                self.right_end_y = height;
            },
            Side::Top => {
                self.top = size;
                self.top_end_x = width;
            },
            Side::Bottom => {
                self.bottom = size;
                self.bottom_end_x = width;
            }
        }

        self
    }
}

