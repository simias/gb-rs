use std::cell::Cell;

use sdl2::event::Event;
use sdl2::keycode::KeyCode;
use sdl2::{joystick, controller};
use sdl2::controller::{GameController, Button, Axis};

use ui::ButtonState;

pub struct Controller {
    buttons:      Cell<::ui::Buttons>,
    #[allow(dead_code)]
    controller:   Option<controller::GameController>,
    x_axis_state: Cell<AxisState>,
    y_axis_state: Cell<AxisState>,
}

impl Controller {
    pub fn new() -> Controller {
        ::sdl2::init(::sdl2::INIT_GAME_CONTROLLER);

        // Attempt to add a game controller

        let njoysticks =
            match joystick::num_joysticks() {
                Ok(n)  => n,
                Err(e) => {
                    error!("Can't enumarate joysticks: {:?}", e);
                    0
                }
            };

        let mut controller = None;

        // For now we just take the first controller we manage to open
        // (if any)
        for id in 0..njoysticks {
            if controller::is_game_controller(id) {
                print!("Attempting to open controller {}", id);

                match GameController::open(id) {
                    Ok(c) => {
                        // We managed to find and open a game controller,
                        // exit the loop
                        println!("Successfully opened \"{}\"", c.name());
                        controller = Some(c);
                        break;
                    },
                    Err(e) => println!("failed: {:?}", e),
                }
            }
        }

        match controller {
            Some(_) => println!("Controller support enabled"),
            None    => println!("No controller found"),
        }

        Controller {
            buttons:      Cell::new(::ui::Buttons::new(ButtonState::Up)),
            controller:   controller,
            x_axis_state: Cell::new(AxisState::Neutral),
            y_axis_state: Cell::new(AxisState::Neutral),
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

    /// Same as update_key but for controller buttons
    fn update_button(&self, button: Button, state: ButtonState) {
        let mut b = self.buttons.get();

        match button {
            Button::A         => b.a      = state,
            Button::B         => b.b      = state,
            Button::DPadLeft  => b.left   = state,
            Button::DPadRight => b.right  = state,
            Button::DPadUp    => b.up     = state,
            Button::DPadDown  => b.down   = state,
            Button::Start     => b.start  = state,
            Button::Back      => b.select = state,
            _                 => (),
        }

        self.buttons.set(b);
    }

    /// Map left stick X/Y to directional buttons
    fn update_axis(&self, axis: Axis, val: i16) {
        let mut b = self.buttons.get();

        let state = AxisState::from_value(val);

        match axis {
            Axis::LeftX => {
                if state != self.x_axis_state.get() {
                    self.x_axis_state.set(state);

                    b.left  = state.down_if_negative();
                    b.right = state.down_if_positive();
                }
            }
            Axis::LeftY => {
                if state != self.y_axis_state.get() {
                    self.y_axis_state.set(state);

                    b.up   = state.down_if_negative();
                    b.down = state.down_if_positive();
                }
            }
            _ => (),
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
                Event::KeyDown { keycode: KeyCode::Escape, .. } =>
                    event = ::ui::Event::PowerOff,
                Event::KeyDown { keycode: key, .. } =>
                    self.update_key(key, ButtonState::Down),
                Event::KeyUp { keycode: key, .. } =>
                    self.update_key(key, ButtonState::Up),
                Event::ControllerButtonDown{ button, .. } =>
                    self.update_button(button, ButtonState::Down),
                Event::ControllerButtonUp{ button, .. } =>
                    self.update_button(button, ButtonState::Up),
                Event::ControllerAxisMotion{ axis, value: val, .. } =>
                    self.update_axis(axis, val),
                Event::Quit { .. } =>
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

#[derive(Copy,PartialEq,Eq)]
enum AxisState {
    Neutral,
    Negative,
    Positive,
}

impl AxisState {
    fn from_value(val: i16) -> AxisState {
        if val > AXIS_DEAD_ZONE {
            AxisState::Positive
        } else if val < -AXIS_DEAD_ZONE {
            AxisState::Negative
        } else {
            AxisState::Neutral
        }
    }

    fn down_if_negative(self) -> ButtonState {
        if self == AxisState::Negative {
            ButtonState::Down
        } else {
            ButtonState::Up
        }
    }

    fn down_if_positive(self) -> ButtonState {
        if self == AxisState::Positive {
            ButtonState::Down
        } else {
            ButtonState::Up
        }
    }
}

/// The controller axis moves in a range from -32768 to +32767. To
/// avoid spurious events this constant says how far from 0 the axis
/// has to move for us to register the event.
const AXIS_DEAD_ZONE: i16 = 10_000;
