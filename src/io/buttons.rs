//! Controller input handling

//! Game boy controls are sampled by setting one of two lines
//! "high". One is connected to the four direction cross, the other to
//! the other four buttons. When the user presses a button it connects
//! one of those two lines with a pin of the gameboy and sets one bit
//! in the INPUT register (if the line is selected).


use std::cell::Cell;
use ui::ButtonState;

pub struct Buttons<'a> {
    /// `true` if the "directions" line is active
    directions_selected: bool,
    /// `true` if the "buttons" line is active
    /// Controller interface
    buttons_selected:    bool,
    /// Abstract interface to the actual UI
    buttons:             &'a Cell<::ui::Buttons>,
}

impl<'a> Buttons<'a> {
    pub fn new<'n>(buttons: &'n Cell<::ui::Buttons>) -> Buttons<'n> {
        Buttons { directions_selected: false,
                  buttons_selected:    false,
                  buttons:             buttons,
        }
    }

    pub fn input(&self) -> u8 {
        let buttons = self.buttons.get();

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
