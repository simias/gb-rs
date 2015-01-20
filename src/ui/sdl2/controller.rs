use std::cell::Cell;

use sdl2::event::Event;
use sdl2::keycode::KeyCode;

use ui::ButtonState;

pub struct Controller {
    buttons: Cell<::ui::Buttons>,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            buttons: Cell::new(::ui::Buttons::new(ButtonState::Up)),
        }
    }

    /// Update key state. For now keybindings are hardcoded.
    fn update_key(&self, key: KeyCode, state: ButtonState) {
        let mut b = self.buttons.get();

        match key {
            KeyCode::Up        => b.up     = state,
            KeyCode::Down      => b.down   = state,
            KeyCode::Left      => b.left   = state,
            KeyCode::Right     => b.right  = state,
            KeyCode::LAlt      => b.a      = state,
            KeyCode::LCtrl     => b.b      = state,
            KeyCode::Return    => b.start  = state,
            KeyCode::RShift    => b.select = state,
            _                  => (),
        }

        self.buttons.set(b);
    }
}

impl ::ui::Controller for Controller {
    fn update(&self) -> ::ui::Event {
        let mut event = ::ui::Event::None;

        loop {
            match ::sdl2::event::poll_event() {
                Event::None =>
                    break,
                Event::KeyDown(_, _, KeyCode::Escape, _, _, _) =>
                    event = ::ui::Event::PowerOff,
                Event::KeyDown(_, _, key, _, _, _) =>
                    self.update_key(key, ButtonState::Down),
                Event::KeyUp(_, _, key, _, _, _) =>
                    self.update_key(key, ButtonState::Up),
                Event::Quit(_) =>
                    event = ::ui::Event::PowerOff,
                _ => ()
            }
        }

        event
    }

    fn buttons(&self) -> &Cell<::ui::Buttons> {
        &self.buttons
    }
}
