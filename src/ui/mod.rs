//! User Interface. Objects used to display the GB Screen, get user
//! input etc...

use std::cell::Cell;

pub mod sdl2;

/// GB screen. Screen resolution is always 160x144
pub trait Display {
    /// Clear the display
    fn clear(&mut self);
    /// Paint pixel at (x, y) using `color`. (0, 0) is top left.
    fn set_pixel(&mut self, x: u32, y: u32, color: ::gpu::Color);
    /// Current frame is done and can be displayed.
    fn flip(&mut self);
}

/// GB controller
pub trait Controller {
    /// Sample the controller input and update internal state.
    fn update(&self) -> Event;
    /// Return a reference to a Cell containing the button state for
    /// use by the emulator code
    fn buttons(&self) -> &Cell<Buttons>;
}

/// Special events that need to be handled synchronously (instead of
/// waiting for the GB program to come check the INPUT register)
pub enum Event {
    /// No event
    None,
    /// Shutdown the emulator
    PowerOff,
}

/// Description of a button's state
#[deriving(Show,Copy)]
pub enum ButtonState {
    /// Key is pushed down
    Down,
    /// Key is up
    Up,
}

/// State of all the GB buttons
#[deriving(Show,Copy)]
pub struct Buttons {
    pub up:        ButtonState,
    pub down:      ButtonState,
    pub left:      ButtonState,
    pub right:     ButtonState,
    pub a:         ButtonState,
    pub b:         ButtonState,
    pub start:     ButtonState,
    pub select:    ButtonState,
    /// State of the interrupt that occurs at the moment a button is
    /// pressed
    pub interrupt: bool,
}

impl Buttons {
    pub fn new(default_state: ButtonState) -> Buttons {
        Buttons {
            a:         default_state,
            b:         default_state,
            start:     default_state,
            select:    default_state,
            up:        default_state,
            down:      default_state,
            left:      default_state,
            right:     default_state,
            interrupt: false,
        }
    }
}
