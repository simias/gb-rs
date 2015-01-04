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

impl ButtonState {
    pub fn is_down(self) -> bool {
        match self {
            ButtonState::Down => true,
            _                 => false,
        }
    }
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

#[cfg(test)]
pub mod dummy {
    //! Dummy implementations of the user interface for use in tests
    //! and benchmarks

    use std::cell::Cell;

    pub struct DummyDisplay;

    impl super::Display for DummyDisplay {
        fn clear(&mut self) {
        }

        fn set_pixel(&mut self, _: u32, _: u32, _: ::gpu::Color) {
        }

        fn flip(&mut self) {
        }
    }

    pub struct DummyController {
        buttons: Cell<super::Buttons>,
    }

    impl DummyController {
        pub fn new() -> DummyController {
            DummyController {
                buttons: Cell::new(super::Buttons::new(super::ButtonState::Up)),
            }
        }
    }

    impl super::Controller for DummyController {
        fn update(&self) -> super::Event {
            super::Event::None
        }

        fn buttons(&self) -> &Cell<super::Buttons> {
            &self.buttons
        }
    }
}
