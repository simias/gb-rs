//! Game Boy GPU emulation

use std::fmt::{Show, Formatter, FormatError};

use ui::Display;

/// GPU state.
pub struct Gpu<'a> {
    /// Emulator Display
    display: &'a mut Display + 'a,
    /// Current line. [0,143] is active video, [144,153] is blanking.
    line: u8,
    /// Position on the current line.
    col: u16,
    /// Object attritube memory
    oam: [u8, ..0xa0],
    /// Video Ram
    vram: [u8, ..0x2000],
    /// `true` if the LCD is enabled.
    enabled: bool,
    /// `true` if we use tile map #1. Otherwise use tile map #0
    window_tile_map_select: bool,
    /// `true` if window display is enabled
    window_display: bool,
    /// `true` if we use tile set #1. Otherwise use tile set #0`
    tile_data_select: bool,
    /// `true` if we use tile map #1. Otherwise use tile map #0
    bg_tile_map_select: bool,
    /// `true` if sprite size is 8x16. Otherwise sprite size is 8x8.
    object_size: bool,
    /// `true` if sprites are displayed
    objects_enabled: bool,
    /// `true` if BG and window display are enabled
    bg_window_enabled: bool,
    /// Background palette
    bgp: u8,
}

/// Current GPU mode
#[deriving(Show)]
pub enum Mode {
    /// In horizontal blanking
    HBlank = 0,
    /// In vertical blanking
    VBlank = 1,
    /// Accessing sprite memory, Sprite attributes RAM [0xfe00, 0xfe9f]
    /// can't be accessed
    Prelude = 2,
    /// Accessing sprite memory and video memory [0x8000, 0x9fff],
    /// both can't be accessed from CPU
    Active = 3,
}

impl<'a> Gpu<'a> {
    /// Create a new Gpu instance.
    pub fn new<'n>(display: &'n mut Display) -> Gpu<'n> {
        Gpu { line:                   0,
              col:                    0,
              oam:                    [0xca, ..0xa0],
              vram:                   [0xca, ..0x2000],
              display:                display,
              enabled:                true,
              window_tile_map_select: false,
              window_display:         false,
              tile_data_select:       true,
              bg_tile_map_select:     false,
              object_size:            false,
              objects_enabled:        false,
              bg_window_enabled:      true,
              bgp:                    0xfc,
        }
    }

    /// Reset the GPU state to power up values
    pub fn reset(&mut self) {
        self.line                   = 0;
        self.col                    = 0;
        self.oam                    = [0xca, ..0xa0];
        self.vram                   = [0xca, ..0x2000];
        self.enabled                = true;
        self.window_tile_map_select = false;
        self.window_display         = false;
        self.tile_data_select       = true;
        self.bg_tile_map_select     = false;
        self.object_size            = false;
        self.objects_enabled        = false;
        self.bg_window_enabled      = true;
        self.bgp                    = 0xfc;
    }

    /// Called at each tick of the system clock. Move the emulated
    /// state one step forward.
    pub fn step(&mut self) {

        //println!("{}", *self);

        if self.col < 456 {
            self.col += 1;
        } else {
            self.col = 0;

            // Move on to the next line
            if self.line < 154 {
                self.line += 1;

                if self.line == 144 {
                    // We're entering blanking, we're done drawing the
                    // current frame
                    self.end_of_frame()
                }

            } else {
                // New frame
                self.line = 0;
            }
        }

        if self.col < 160 && self.line < 144 {
            let x = self.col as u8;
            let y = self.line;

            self.render_pixel(x, y);
        }
    }

    /// Return current GPU mode
    pub fn get_mode(&self) -> Mode {
        if self.line < 144 {
            match self.col {
                0  ... 79  => Prelude,
                80 ... 172 => Active,
                _          => HBlank,
            }
        } else {
            VBlank
        }
    }

    /// Handle reconfig through LCDC register
    pub fn set_lcdc(&mut self, lcdc: u8) {
        self.enabled                = lcdc & 0x80 != 0;
        self.window_tile_map_select = lcdc & 0x40 != 0;
        self.window_display         = lcdc & 0x20 != 0;
        self.tile_data_select       = lcdc & 0x10 != 0;
        self.bg_tile_map_select     = lcdc & 0x08 != 0;
        self.object_size            = lcdc & 0x04 != 0;
        self.objects_enabled        = lcdc & 0x02 != 0;
        self.bg_window_enabled      = lcdc & 0x01 != 0;
    }

    pub fn set_bgp(&mut self, bgp: u8) {
        self.bgp = bgp;
    }

    pub fn bgp(&self) -> u8 {
        self.bgp
    }

    /// Return number of line currently being drawn
    pub fn get_line(&self) -> u8 {
        self.line
    }

    /// Called when the last line of the active display has been drawn
    fn end_of_frame(&mut self) {
        self.display.flip();
    }

    /// Get byte from VRAM
    pub fn get_vram(&self, addr: u16) -> u8 {
        self.vram[addr as uint]
    }

    /// Set byte in VRAM
    pub fn set_vram(&mut self, addr: u16, val: u8) {
        self.vram[addr as uint] = val;
    }

    /// Get byte from OAM
    pub fn get_oam(&self, addr: u16) -> u8 {
        self.oam[addr as uint]
    }

    /// Set byte in OAM
    pub fn set_oam(&mut self, addr: u16, val: u8) {
        self.oam[addr as uint] = val;
    }

    fn render_pixel(&mut self, x: u8, y: u8) {
        let tile_map_x = x / 8;
        let tile_map_y = y / 8;
        let tile_x     = x % 8;
        let tile_y     = y % 8;

        // The screen is divided in 8x8 pixel tiles. It creates a
        // matrix of 32x32 tiles (As far as the GPU is concerned the
        // screen resolution is 256x256). The tile map contains one u8
        // per tile which is the index of the tile to use in the tile
        // set.
        let tile_index = self.bg_tile_index(tile_map_x, tile_map_y);

        let tile_pix_value = self.get_pix_value(tile_index, tile_x, tile_y);

        // Use tile_pix_value as index in the bgp
        let pix_value = (self.bgp >> ((tile_pix_value * 2) as uint)) & 0x3;

        self.display.set_pixel(x as u32, y as u32, pix_value);
    }

    /// Return the background tile index for the tile at (`tx`, `ty`)
    fn bg_tile_index(&self, tx: u8, ty: u8) -> u8 {
        let base = match self.bg_tile_map_select {
            false  => 0x1800,
            true   => 0x1c00,
        };

        let tx = tx as u16;
        let ty = ty as u16;

        let map_addr = base + (ty * 32) + tx;

        self.vram[map_addr as uint]
    }

    /// Get the value of pixel (`x`, `y`) in `tile`. Return a value
    /// between 0 and 3.
    fn get_pix_value(&self, tile: u8, x: u8, y: u8) -> u8 {

        if x >= 8 || y >= 8 {
            panic!("tile pos out of range");
        }

        let base = match self.tile_data_select {
            // If tile_data_select is false `tile` is signed and in
            // the range [-128, 127]. Tile 0 is at 0x9000.
            false => (0x1000 + (((tile as i8) as i16) * 16)) as u16,
            // Otherwise it's unsigned and starts at 0x8000
            true  => 0x0 + (tile as u16) * 16,
        };

        let addr = base + 2 * (y as u16);

        let addr = addr    as uint;
        let x    = (7 - x) as uint;

        // Each row of 8 pixels is split across two contiguous bytes:
        // the first for the LSB, the 2nd for the MSB
        let lsb = (self.vram[addr]     >> x) & 1;
        let msb = (self.vram[addr + 1] >> x) & 1;

        msb << 1 | lsb
    }
}

impl<'a> Show for Gpu<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        try!(write!(f, "Gpu at ({}, {}) [{}] ", self.col, self.line, self.get_mode()));

        Ok(())
    }
}
