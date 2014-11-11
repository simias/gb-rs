//! User Interface. Objects used to display the GB Screen, get user
//! input etc...

pub mod sdl2;

/// GB screen. Screen resolution is always 160x144
pub trait Display {
    /// Clear the display
    fn clear(&mut self);
    /// Set pixel at (x, y). (0, 0) is top left. col is in the range
    /// [0, 3] where 0 is white and 3 is black.
    fn set_pixel(&mut self, x: u32, y: u32, col: u8);
    /// Current frame is done and can be displayed.
    fn flip(&mut self);
}

/// Dummy Display that does nothing, for testing purposes
#[allow(dead_code)]
pub struct DummyDisplay;

impl Display for DummyDisplay {
    fn clear(&mut self) {
    }

    fn set_pixel(&mut self, _: u32, _: u32, _: u8) {
    }

    fn flip(&mut self) {
    }
}
