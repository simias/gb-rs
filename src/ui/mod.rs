//! User Interface. Objects used to display the GB Screen, get user
//! input etc...

pub mod sdl2;

/// GB screen. Screen resolution is always 160x144
pub trait Display {
    /// Clear the display
    fn clear(&mut self);
    /// Set or clear pixel at (x, y). (0, 0) is top left.
    fn set_pixel(&mut self, x: u32, y: u32, set: bool);
    /// Current frame is done and can be displayed.
    fn flip(&mut self);
}

/// Dummy Display that does nothing, for testing purposes
#[allow(dead_code)]
pub struct DummyDisplay;

impl Display for DummyDisplay {
    fn clear(&mut self) {
    }

    fn set_pixel(&mut self, _: u32, _: u32, _: bool) {
    }

    fn flip(&mut self) {
    }
}
