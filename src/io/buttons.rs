//! Controller input handling

//! Game boy controls are sampled by setting one of two lines
//! "high". One is connected to the four direction cross, the other to
//! the other four buttons. When the user presses a button it connects
//! one of those two lines with a pin of the gameboy and sets one bit
//! in the INPUT register (if the line is selected).


use ui::{Controller, ButtonState};

pub struct Buttons<'a> {
    /// Counter to the next controller update
    next_update:         u32,
    /// `true` if the "directions" line is active
    directions_selected: bool,
    /// `true` if the "buttons" line is active
    /// Controller interface
    buttons_selected:    bool,
    /// Abstract interface to the actual UI
    controller:          &'a mut (Controller + 'a),
}

impl<'a> Buttons<'a> {
    pub fn new<'n>(controller: &'n mut (Controller + 'n)) -> Buttons<'n> {
        Buttons { next_update:         0,
                  directions_selected: false,
                  buttons_selected:    false,
                  controller:          controller,
        }
    }

    pub fn step(&mut self) -> ::ui::Event {
        let r =
            if self.next_update == 0 {
                self.controller.update()
            } else {
                ::ui::Event::None
            };

        self.next_update = (self.next_update + 1) % UPDATE_FREQ;

        r
    }

    pub fn input(&self) -> u8 {
        let buttons = self.controller.state();

        let mut r = 0;

        if self.directions_selected {
            r |= match buttons.right {
                ButtonState::Up => 1,
                _               => 0,
            } << 0;

            r |= match buttons.left {
                ButtonState::Up => 1,
                _               => 0,
            } << 1;

            r |= match buttons.up {
                ButtonState::Up => 1,
                _               => 0,
            } << 2;


            r |= match buttons.down {
                ButtonState::Up => 1,
                _               => 0,
            } << 3;
        }

        if self.buttons_selected {
            r |= match buttons.a {
                ButtonState::Up => 1,
                _               => 0,
            } << 0;

            r |= match buttons.b {
                ButtonState::Up => 1,
                _               => 0,
            } << 1;

            r |= match buttons.select {
                ButtonState::Up => 1,
                _               => 0,
            } << 2;


            r |= match buttons.start {
                ButtonState::Up => 1,
                _               => 0,
            } << 3;
        }

        r
    }

    pub fn set_input(&mut self, val: u8)  {
        // We select the lines by setting the bit to 0
        self.directions_selected = val & 0x10 == 0;
        self.buttons_selected    = val & 0x20 == 0;
    }
}

/// How often the controller state gets updated, counted in sysclk
/// cycles
const UPDATE_FREQ: u32 = 0x10000;

/*
                let v = self.io[0];

                let buttons = self.controller.state();

                let mut r = 0;

                if v & 0x10 == 0 {
                    r |= match buttons.right {
                        ButtonState::Up => 1,
                        _               => 0,
                    } << 0;

                    r |= match buttons.left {
                        ButtonState::Up => 1,
                        _               => 0,
                    } << 1;

                    r |= match buttons.up {
                        ButtonState::Up => 1,
                        _               => 0,
                    } << 2;


                    r |= match buttons.down {
                        ButtonState::Up => 1,
                        _               => 0,
                    } << 3;
                }

                if v & 0x20 == 0 {
                    r |= match buttons.a {
                        ButtonState::Up => 1,
                        _               => 0,
                    } << 0;

                    r |= match buttons.b {
                        ButtonState::Up => 1,
                        _               => 0,
                    } << 1;

                    r |= match buttons.select {
                        ButtonState::Up => 1,
                        _               => 0,
                    } << 2;


                    r |= match buttons.start {
                        ButtonState::Up => 1,
                        _               => 0,
                    } << 3;
                }

                return r;
*/
