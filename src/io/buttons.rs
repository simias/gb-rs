//! Controller input handling

//! Game boy controls are sampled by setting one of two lines
//! "high". One is connected to the four direction cross, the other to
//! the other four buttons. When the user presses a button it connects
//! one of those two lines with a pin of the gameboy and sets one bit
//! in the INPUT register (if the line is selected).


use std::cell::Cell;

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

    /// Return the value of the INPUT register. Lines are to 1 when
    /// inactive.
    pub fn input(&self) -> u8 {
        let buttons = self.buttons.get();

        // For simplicity we'll mark the active lines with 1 and
        // invert the value at the end
        let mut active = 0;

        if self.directions_selected {
            active |= 0x10;

            active |= (buttons.right.is_down() as u8) << 0;
            active |= (buttons.left .is_down() as u8) << 1;
            active |= (buttons.up   .is_down() as u8) << 2;
            active |= (buttons.down .is_down() as u8) << 3;
        }

        if self.buttons_selected {
            active |= 0x20;

            active |= (buttons.a     .is_down() as u8) << 0;
            active |= (buttons.b     .is_down() as u8) << 1;
            active |= (buttons.select.is_down() as u8) << 2;
            active |= (buttons.start .is_down() as u8) << 3;
        }

        // Now we can complement the value and return it
        !active
    }

    pub fn set_input(&mut self, val: u8)  {
        // We select the lines by setting the bit to 0
        self.directions_selected = val & 0x10 == 0;
        self.buttons_selected    = val & 0x20 == 0;
    }
}
