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
    /// Line compare
    lyc: u8,
    /// VBlank interrupt status
    it_vblank: bool,
    /// LYC match interrupt enable (IT when LY == LYC)
    iten_lyc: bool,
    /// Interrupt during prelude (mode == 2)
    iten_prelude: bool,
    /// Interrupt during vblank (mode == 1). This is not the same as
    /// `it_vblank` above: it_vblank fires with a higher priority and
    /// is not shared with other interrupt sources like this one.
    iten_vblank: bool,
    /// Interrupt during hblank (mode == 0)
    iten_hblank: bool,
    /// Lcdc interrupt status
    lcd_it_status: LcdItStatus,
    /// Background y position
    scy: u8,
    /// Background x position
    scx: u8,
}

/// Current GPU mode
#[deriving(Show, PartialEq)]
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

/// State of the LCD interrupt (as controlled by the STAT
/// register).
///
/// I'm not absolutely certain I got things right but if I understand
/// correctly: the interrupt source is configurable (LYC, prelude,
/// vblank, hblank). The way I see it all those interrupt sources are
/// ORed together and an interrupt is only signaled on a rising edge
/// of the ORed signal.
///
/// So for instance if the LYC and HBlank interrupts are enabled and
/// we're at the matched line, the interrupt will trigger at the
/// beginning of the line (LY == LYC) but not at the beginning of
/// hblank (since the IT line is already high).

/// However, if the LYC register value is changed in the middle of the
/// line and the LY == LYC is no longer true, the IT signal will go
/// low and can be triggered again in the same line.
#[deriving(PartialEq)]
enum LcdItStatus {
    /// Interrupt is inactive
    Inactive,
    /// Interrupt event occured
    Triggered,
    /// Interrupt event occured and has been acknowledged. It will be
    /// rearmed when the signal goes low.
    Acked,
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
              lyc:                    0x00,
              it_vblank:              false,
              iten_lyc:               false,
              iten_prelude:           false,
              iten_vblank:            false,
              iten_hblank:            false,
              lcd_it_status:          Inactive,
              scy:                    0,
              scx:                    0,
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
        self.lyc                    = 0;
        self.it_vblank              = false;
        self.it_vblank              = false;
        self.iten_lyc               = false;
        self.iten_prelude           = false;
        self.iten_vblank            = false;
        self.iten_hblank            = false;
        self.lcd_it_status          = Inactive;
        self.scy                    = 0;
        self.scx                    = 0;
    }

    /// Called at each tick of the system clock. Move the emulated
    /// state one step forward.
    pub fn step(&mut self) {

        //println!("{}", *self);

        if self.col < timings::HTOTAL {
            self.col += 1;
        } else {
            // Move on to the next line
            self.col = 0;

            if self.line < timings::VTOTAL {
                self.line += 1;

                if self.line == timings::VSYNC_ON {
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

            if self.bg_window_enabled {
                self.render_pixel(x, y);
            }
        }

        self.update_ldc_interrupt();
    }

    /// Return current GPU mode
    pub fn get_mode(&self) -> Mode {
        if self.line < timings::VSYNC_ON {
            if self.col < timings::HACTIVE_ON {
                Prelude
            } else if self.col < timings::HSYNC_ON {
                Active
            } else {
                HBlank
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

    /// Generate value of lcdc register
    pub fn lcdc(&self) -> u8 {
        let mut r = 0;

        r |= (self.enabled                as u8) << 7;
        r |= (self.window_tile_map_select as u8) << 6;
        r |= (self.window_display         as u8) << 5;
        r |= (self.tile_data_select       as u8) << 4;
        r |= (self.bg_tile_map_select     as u8) << 3;
        r |= (self.object_size            as u8) << 2;
        r |= (self.objects_enabled        as u8) << 1;
        r |= (self.bg_window_enabled      as u8) << 0;

        r
    }

    pub fn stat(&self) -> u8 {
        let mut r = 0;

        let c = self.lyc == self.line;

        r |= (self.iten_lyc     as u8) << 6;
        r |= (self.iten_prelude as u8) << 5;
        r |= (self.iten_vblank  as u8) << 4;
        r |= (self.iten_hblank  as u8) << 3;
        r |= (c                 as u8) << 2;
        r |= self.get_mode()    as u8;

        r
    }

    pub fn set_stat(&mut self, stat: u8) {
        self.iten_lyc     = stat & 0x40 != 0;
        self.iten_prelude = stat & 0x20 != 0;
        self.iten_vblank  = stat & 0x10 != 0;
        self.iten_hblank  = stat & 0x03 != 0;
        // Other fields are R/O

        // Update interrupt status with new stat params
        self.update_ldc_interrupt();
    }

    /// Reconfiguration of SCY register
    pub fn scy(&self) -> u8 {
        self.scy
    }

    /// Return value of SCY register
    pub fn set_scy(&mut self, scy: u8) {
        self.scy = scy;
    }

    /// Reconfiguration of SCX register
    pub fn scx(&self) -> u8 {
        self.scx
    }

    /// Return value of SCX register
    pub fn set_scx(&mut self, scx: u8) {
        self.scx = scx;
    }

    /// Handle reconfiguration of the lyc register
    pub fn set_lyc(&mut self, lyc: u8) {
        self.lyc = lyc;
    }

    /// Return value of the lyc register
    pub fn lyc(&self) -> u8 {
        self.lyc
    }

    /// Handle reconfiguration of the background palette
    pub fn set_bgp(&mut self, bgp: u8) {
        self.bgp = bgp;
    }

    /// Return value of the background palette register
    pub fn bgp(&self) -> u8 {
        self.bgp
    }

    /// Return number of line currently being drawn
    pub fn line(&self) -> u8 {
        self.line
    }

    /// Called when the last line of the active display has been drawn
    fn end_of_frame(&mut self) {
        self.it_vblank = true;
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

    /// Return status of VBlank interrupt
    pub fn it_vblank(&self) -> bool {
        self.it_vblank
    }

    /// Acknowledge VBlank interrupt
    pub fn ack_it_vblank(&mut self) {
        self.it_vblank = false;
    }

    /// Force VBlank interrupt state
    pub fn force_it_vblank(&mut self, set: bool) {
        self.it_vblank = set;
    }

    /// Return status of Lcd interrupt
    pub fn it_lcd(&self) -> bool {
        self.lcd_it_status == Triggered
    }

    /// Acknowledge Lcd interrupt
    pub fn ack_it_lcd(&mut self) {
        if self.lcd_it_status == Triggered {
            self.lcd_it_status = Acked;
        }
    }

    /// Force Lcd interrupt state. As with all the rest of the Lcd
    /// interrupt state machine, I'm not sure if that's right.
    pub fn force_it_lcd(&mut self, set: bool) {
        match set {
            true  => self.lcd_it_status = Triggered,
            false => self.ack_it_lcd(),
        }
    }

    /// Return the current level of the LCD interrupt (`true` if one
    /// of the interrupt conditions is met and is enabled).
    fn lcd_interrupt_level(&self) -> bool {
        let mode = self.get_mode();

        (self.iten_lyc     && self.lyc == self.line) ||
        (self.iten_prelude && mode == Prelude)       ||
        (self.iten_vblank  && mode == VBlank)        ||
        (self.iten_hblank  && mode == HBlank)
    }

    /// Look for a transition in the LCD interrupt to see if we should
    /// trigger a new one (or rearm it)
    fn update_ldc_interrupt(&mut self) {
        let level = self.lcd_interrupt_level();

        match level {
            true => {
                if self.lcd_it_status == Inactive {
                    // Rising edge of IT line, we trigger a new interrupt.
                    self.lcd_it_status = Triggered;
                }
            }
            false => {
                // Not entirely sure about that one. If the interrupt
                // has not been acked yet, what should be done? At the
                // moment I just assume it's shadowed somewhere and
                // won't go down until acked.
                if self.lcd_it_status == Acked {
                    // IT line returned to low, it could trigger again
                    // within the same line.
                    self.lcd_it_status = Inactive;
                }
            }
        }
    }

    fn render_pixel(&mut self, x: u8, y: u8) {
        let bgx        = x + self.scx;
        let bgy        = y + self.scy;
        let tile_map_x = bgx / 8;
        let tile_map_y = bgy / 8;
        let tile_x     = bgx % 8;
        let tile_y     = bgy % 8;

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

mod timings {
    //! LCD timings

    /// Total line size (including hblank)
    pub const HTOTAL:     u16 = 456;
    /// Beginning of Active period
    pub const HACTIVE_ON: u16 = 80;
    /// Beginning of HSync period
    pub const HSYNC_ON:   u16 = 173;

    /// Total number of lines (including vblank)
    pub const VTOTAL:   u8 = 154;
    /// Beginning of VSync period
    pub const VSYNC_ON: u8 = 144;
}
