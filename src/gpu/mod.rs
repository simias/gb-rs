//! Game Boy GPU emulation

use std::fmt::{Show, Formatter, Error};
use ui::Display;
use gpu::sprite::Sprite;

mod sprite;

/// GPU state.
pub struct Gpu<'a> {
    /// Emulator Display
    display: &'a mut (Display + 'a),
    /// Current line. [0,143] is active video, [144,153] is blanking.
    line: u8,
    /// Counter for the horizontal period
    htick: u16,
    /// Object attritube memory
    oam: [Sprite, ..0xa0],
    /// Video Ram
    vram: [u8, ..0x2000],
    /// `true` if the LCD is enabled.
    enabled: bool,
    /// Which tile map the window uses
    window_tile_map: TileMap,
    /// `true` if window display is enabled
    window_enabled: bool,
    /// Which tile set Background and Window use
    bg_win_tile_set: TileSet,
    /// Which tile map background uses
    bg_tile_map: TileMap,
    /// Resolution of the sprites
    sprite_size: SpriteSize,
    /// `true` if sprites are displayed
    sprites_enabled: bool,
    /// `true` if background display is enabled
    bg_enabled: bool,
    /// Background palette
    bgp: Palette,
    /// Object palette 0
    obp0: Palette,
    /// Object palette 1
    obp1: Palette,
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
    /// Window top-left x position + 7
    wx: u8,
    /// Window top-left y position.
    wy: u8,
    /// Sprites displayed on each line. Contains an index into OAM or
    /// None. There can't be more than 10 sprites displayed on each
    /// line.
    line_cache: [[Option<u8>, ..10], ..144],
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
              htick:                  0,
              oam:                    [Sprite::new(), ..0xa0],
              vram:                   [0xca, ..0x2000],
              display:                display,
              enabled:                false,
              window_tile_map:        TileMap::Low,
              window_enabled:         false,
              bg_win_tile_set:        TileSet::Set0,
              bg_tile_map:            TileMap::Low,
              sprite_size:            SpriteSize::Sz8x8,
              sprites_enabled:        false,
              bg_enabled:             false,
              bgp:                    Palette::from_reg(0xff),
              obp0:                   Palette::from_reg(0xff),
              obp1:                   Palette::from_reg(0xff),
              lyc:                    0x00,
              it_vblank:              false,
              iten_lyc:               false,
              iten_prelude:           false,
              iten_vblank:            false,
              iten_hblank:            false,
              lcd_it_status:          LcdItStatus::Inactive,
              scy:                    0,
              scx:                    0,
              wx:                     0,
              wy:                     0,
              line_cache:             [[None, ..10], ..144],
        }
    }

    /// Called at each tick of the system clock. Move the emulated
    /// state one step forward.
    pub fn step(&mut self) {

        if !self.enabled {
            return;
        }

        self.htick = (self.htick + 1) % timings::HTOTAL;

        if self.htick == timings::HSYNC_ON {
            // Entering horizontal blanking

            self.line = (self.line + 1) % timings::VTOTAL;

            if self.line == timings::VSYNC_ON {
                // We're entering vertical blanking, we're done drawing the
                // current frame
                self.end_of_frame()
            }
        }

        // Compute at which cycle the first pixel will actually be
        // output on the screen. I don't know where this comes from
        // but it's what GearBoy seems to use. Using 48 for the first
        // line messes up The Legend of Zelda's intro.
        let line_start = match self.line {
                0 => 160,
                _ => 48,
        };

        if self.htick == line_start && self.line < timings::VSYNC_ON {
            // It's time to draw the current line

            let y = self.line;

            for x in range(0, 160) {
                self.render_pixel(x, y);
            }
        }
        self.update_ldc_interrupt();
    }

    /// Return current GPU mode
    pub fn mode(&self) -> Mode {
        if self.line < timings::VSYNC_ON {
            if self.htick < timings::HACTIVE_ON {
                Mode::Prelude
            } else if self.htick < timings::HSYNC_ON {
                Mode::Active
            } else {
                Mode::HBlank
            }
        } else {
            Mode::VBlank
        }
    }

    /// Handle reconfig through LCDC register
    pub fn set_lcdc(&mut self, lcdc: u8) {
        self.enabled          = lcdc & 0x80 != 0;
        self.window_tile_map  = match lcdc & 0x40 != 0 {
            true  => TileMap::High,
            false => TileMap::Low,
        };
        self.window_enabled  = lcdc & 0x20 != 0;
        self.bg_win_tile_set = match lcdc & 0x10 != 0 {
            true  => TileSet::Set1,
            false => TileSet::Set0,
        };
        self.bg_tile_map     = match lcdc & 0x08 != 0 {
            true  => TileMap::High,
            false => TileMap::Low,
        };
        let new_sprite_size = match lcdc & 0x04 != 0 {
            false => SpriteSize::Sz8x8,
            true  => SpriteSize::Sz8x16,
        };

        self.sprites_enabled = lcdc & 0x02 != 0;
        self.bg_enabled      = lcdc & 0x01 != 0;

        if !self.enabled {
            // Reset to the first pixel to start back here once we're
            // re-enabled.
            self.line  = 0;
            self.htick = 0;
        }

        if new_sprite_size != self.sprite_size {
            self.sprite_size = new_sprite_size;

            self.rebuild_line_cache();
        }
    }

    /// Generate value of lcdc register
    pub fn lcdc(&self) -> u8 {
        let mut r = 0;

        r |= (self.enabled         as u8) << 7;
        r |= match self.window_tile_map {
            TileMap::High => 1,
            TileMap::Low  => 0,
        } << 6;
        r |= (self.window_enabled  as u8) << 5;
        r |= match self.bg_win_tile_set {
            TileSet::Set1 => 1,
            TileSet::Set0 => 0
        }<< 4;
        r |= match self.bg_tile_map {
            TileMap::High => 1,
            TileMap::Low  => 0,
        } << 3;
        r |= match self.sprite_size {
            SpriteSize::Sz8x16 => 1,
            SpriteSize::Sz8x8  => 0,
        } << 2;
        r |= (self.sprites_enabled as u8) << 1;
        r |= (self.bg_enabled      as u8) << 0;

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

        // Apparently mode is 0 when disabled
        if self.enabled {
            r |= self.mode() as u8;
        }

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
        self.bgp = Palette::from_reg(bgp);
    }

    /// Return value of the background palette register
    pub fn bgp(&self) -> u8 {
        self.bgp.into_reg()
    }

    /// Handle reconfiguration of the sprite palette 0
    pub fn set_obp0(&mut self, obp0: u8) {
        self.obp0 = Palette::from_reg(obp0);
    }

    /// Return value of the background palette register
    pub fn obp0(&self) -> u8 {
        self.obp0.into_reg()
    }

    /// Handle reconfiguration of the sprite palette 1
    pub fn set_obp1(&mut self, obp1: u8) {
        self.obp1 = Palette::from_reg(obp1);
    }

    /// Return value of the background palette register
    pub fn obp1(&self) -> u8 {
        self.obp1.into_reg()
    }

    /// Return number of line currently being drawn
    pub fn line(&self) -> u8 {
        self.line
    }

    /// Return value of wy register
    pub fn wy(&self) -> u8 {
        self.wy
    }

    /// Handle reconfiguration of wy register
    pub fn set_wy(&mut self, wy: u8) {
        self.wy = wy
    }

    /// Return value of wx register
    pub fn wx(&self) -> u8 {
        self.wx
    }

    /// Handle reconfiguration of wx register
    pub fn set_wx(&mut self, wx: u8) {
        self.wx = wx
    }

    /// Called when the last line of the active display has been drawn
    fn end_of_frame(&mut self) {
        self.it_vblank = true;
        self.display.flip();
    }

    /// Get byte from VRAM
    pub fn vram(&self, addr: u16) -> u8 {
        self.vram[addr as uint]
    }

    /// Set byte in VRAM
    pub fn set_vram(&mut self, addr: u16, val: u8) {
        self.vram[addr as uint] = val;
    }

    /// Get byte from OAM
    pub fn oam(&self, addr: u16) -> u8 {
        // Each sprite takes 4 byte in OAM
        let index     = (addr / 4) as uint;
        let attribute = addr % 4;

        let sprite = self.sprite(index);

        match attribute {
            0 => sprite.y_pos(),
            1 => sprite.x_pos(),
            2 => sprite.tile(),
            3 => sprite.flags(),
            _ => panic!("unreachable"),
        }
    }

    /// Set byte in OAM
    pub fn set_oam(&mut self, addr: u16, val: u8) {
        // Each sprite takes 4 byte in OAM
        let index     = (addr / 4) as uint;
        let attribute = addr % 4;

        let update_cache = {
            let sprite = self.sprite_mut(index);

            match attribute {
                0 => {
                    if sprite.y_pos() != val {
                        sprite.set_y_pos(val);
                        true
                    } else {
                        false
                    }
                }
                1 => {
                    if sprite.x_pos() != val {
                        sprite.set_x_pos(val);
                        true
                    } else {
                        false
                    }
                }
                2 => {
                    sprite.set_tile(val);
                    false
                }
                3 => {
                    sprite.set_flags(val);
                    false
                }
                _ => panic!("unreachable"),
            }
        };

        // We need to invalidate the cache only if the sprite location
        // has changed
        if update_cache {
            self.rebuild_line_cache();
        }
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
        self.lcd_it_status == LcdItStatus::Triggered
    }

    /// Acknowledge Lcd interrupt
    pub fn ack_it_lcd(&mut self) {
        if self.lcd_it_status == LcdItStatus::Triggered {
            self.lcd_it_status = LcdItStatus::Acked;
        }
    }

    /// Force Lcd interrupt state. As with all the rest of the Lcd
    /// interrupt state machine, I'm not sure if that's right.
    pub fn force_it_lcd(&mut self, set: bool) {
        match set {
            true  => self.lcd_it_status = LcdItStatus::Triggered,
            false => self.ack_it_lcd(),
        }
    }

    /// Return the current level of the LCD interrupt (`true` if one
    /// of the interrupt conditions is met and is enabled).
    fn lcd_interrupt_level(&self) -> bool {
        let mode = self.mode();

        (self.iten_lyc     && self.lyc == self.line) ||
            (self.iten_prelude && mode == Mode::Prelude) ||
            (self.iten_vblank  && mode == Mode::VBlank)  ||
            (self.iten_hblank  && mode == Mode::HBlank)
    }

    /// Look for a transition in the LCD interrupt to see if we should
    /// trigger a new one (or rearm it)
    fn update_ldc_interrupt(&mut self) {
        let level = self.lcd_interrupt_level();

        match level {
            true => {
                if self.lcd_it_status == LcdItStatus::Inactive {
                    // Rising edge of IT line, we trigger a new interrupt.
                    self.lcd_it_status = LcdItStatus::Triggered;
                }
            }
            false => {
                // Not entirely sure about that one. If the interrupt
                // has not been acked yet, what should be done? At the
                // moment I just assume it's shadowed somewhere and
                // won't go down until acked.
                if self.lcd_it_status == LcdItStatus::Acked {
                    // IT line returned to low, it could trigger again
                    // within the same line.
                    self.lcd_it_status = LcdItStatus::Inactive;
                }
            }
        }
    }

    fn sprite(&self, index: uint) -> &Sprite {
        &self.oam[index]
    }

    fn sprite_mut(&mut self, index: uint) -> &mut Sprite {
        &mut self.oam[index]
    }

    /// Return `true` if the pixel at (`x`, `y`) is in the window
    fn in_window(&self, x: u8, y: u8) -> bool {
        let x  = x as i32;
        let y  = y as i32;
        let wx = (self.wx as i32) - 7;
        let wy = self.wy as i32;

        x >= wx && y >= wy
    }

    /// Get pixel in the window. Assumes (`x`, `y`) is inside the
    /// window.
    fn window_color(&mut self, x: u8, y: u8) -> AlphaColor {
        // Window X value is offset by 7 for some reason
        let px = x - self.wx + 7;
        let py = y - self.wy;

        let map = self.window_tile_map;
        let set = self.bg_win_tile_set;

        self.bg_win_color(px, py, map, set)
    }

    fn background_color(&mut self, x: u8, y: u8) -> AlphaColor {
        let px = x + self.scx;
        let py = y + self.scy;

        let map = self.bg_tile_map;
        let set = self.bg_win_tile_set;

        self.bg_win_color(px, py, map, set)
    }

    /// Get one pixel from either the window or the background.
    fn bg_win_color(&self,
                    x: u8,
                    y: u8,
                    map: TileMap,
                    set: TileSet) -> AlphaColor {
        let tile_map_x = x / 8;
        let tile_map_y = y / 8;
        let tile_x     = x % 8;
        let tile_y     = y % 8;

        // The screen is divided in 8x8 pixel tiles. It creates a
        // matrix of 32x32 tiles (As far as the GPU is concerned the
        // screen resolution is 256x256). The tile map contains one u8
        // per tile which is the index of the tile to use in the tile
        // set.
        let tile_index = self.tile_index(tile_map_x, tile_map_y, map);

        let tile_color = self.pix_color(tile_index, tile_x, tile_y, set);

        AlphaColor {
            // Transform tile_color through the palette
            color:  self.bgp.transform(tile_color),
            // The pixel is transparent if the value pre-palette is white
            opaque: tile_color != Color::White,
        }
    }

    /// Return the tile index for the tile at (`tx`, `ty`) in `map`
    fn tile_index(&self, tx: u8, ty: u8, map: TileMap) -> u8 {
        let base = map.base();

        let tx = tx as u16;
        let ty = ty as u16;

        let map_addr = base + (ty * 32) + tx;

        self.vram[map_addr as uint]
    }

    /// Get the color of pixel (`x`, `y`) in `tile`.
    fn pix_color(&self, tile: u8, x: u8, y: u8, set: TileSet) -> Color {

        if x >= 8 || y >= 16 {
            panic!("tile pos out of range ({}, {})", x, y);
        }

        let base = set.tile_addr(tile);

        let addr = base + 2 * (y as u16);

        let addr = addr    as uint;
        let x    = (7 - x) as uint;

        // Each row of 8 pixels is split across two contiguous bytes:
        // the first for the LSB, the 2nd for the MSB
        let lsb = (self.vram[addr]     >> x) & 1;
        let msb = (self.vram[addr + 1] >> x) & 1;

        Color::from_u8(msb << 1 | lsb)
    }

    /// Rebuild the entire Sprite cache for each line. This is pretty
    /// expensive.
    fn rebuild_line_cache(&mut self) {
        // Clear the cache
        self.line_cache = [[None, ..10], ..144];

        // Rebuild it
        for i in range(0, self.oam.len()) {
            self.cache_sprite(i as u8);
        }
    }

    /// Insert sprite at `index` into the line cache
    fn cache_sprite(&mut self, index: u8) {
        let sprite = self.oam[index as uint];
        let height = self.sprite_size.height();
        let start  = sprite.top_line();
        let end    = start + (height as i32);

        for y in range(start, end) {
            if y < 0 || y >= 144 {
                // Sprite line is not displayed
                continue;
            }

            let y = y as uint;

            let l = self.line_cache[y].len();

            if self.line_cache[y][l - 1].is_some() {
                // We reached the sprite limit for that line, we can
                // display no more.
                continue;
            }

            // Insert sprite into the cache for this line. We order
            // the sprites from left to right and from highest to
            // lowest priority.
            for i in range(0u, l) {
                match self.line_cache[y][i] {
                    None => {
                        // This cache entry is empty, use it to hold
                        // our sprite and move on to the next line
                        self.line_cache[y][i] = Some(index);
                        break;
                    }
                    Some(other) => {
                        let other_sprite = &self.oam[other as uint];

                        // When sprites overlap the one with the
                        // smallest x pos is on top. If the x values
                        // are equal then the offset in OAM is used.
                        if sprite.x_pos() < other_sprite.x_pos() ||
                           (sprite.x_pos() == other_sprite.x_pos() &&
                            index < other) {
                            // Our sprite is higher priority, move the
                            // rest of the cacheline one place. We
                            // know that the last item is None since
                            // it's checked above.
                            for j in range(i, l - 1).rev() {
                                self.line_cache[y][j + 1] =
                                    self.line_cache[y][j];
                            }

                            self.line_cache[y][i] = Some(index);
                            break;
                        }
                    }
                }
            }
        }
    }

    /// Render a single pixel from the display
    fn render_pixel(&mut self, x: u8, y: u8) {
        let bg_col =
            // Window is always on top of background
            if self.window_enabled && self.in_window(x, y) {
                self.window_color(x, y)
            } else if self.bg_enabled && self.bg_enabled {
                self.background_color(x, y)
            } else {
                // No background or window
                AlphaColor { color: Color::White, opaque: false }
            };

        let col = if self.sprites_enabled {
            self.render_sprite(x, y, bg_col)
        } else {
            bg_col.color
        };

        self.display.set_pixel(x as u32, y as u32, col);
    }

    fn render_sprite(&self, x: u8, y: u8, bg_col: AlphaColor) -> Color {

        for &entry in self.line_cache[y as uint].iter() {
            match entry {
                None        => break, // Nothing left in cache
                Some(index) => {
                    let sprite = &self.oam[index as uint];

                    let sprite_x = (x as i32) - sprite.left_column();

                    if sprite_x >= 8 {
                        // Sprite was earlier on the line
                        continue
                    }

                    if sprite_x < 0 {
                        // It's too early to draw that sprite. Since
                        // sprites are in order on the line we know
                        // there's no sprite remaining to be drawn
                        break;
                    }

                    if sprite.background() && bg_col.opaque {
                        // Sprite is covered by the background
                        continue;
                    }

                    let sprite_y = (y as i32) - sprite.top_line();

                    let (height, tile) = match self.sprite_size {
                        SpriteSize::Sz8x8  => (7, sprite.tile()),
                        // For 16pix tiles the LSB is ignored
                        SpriteSize::Sz8x16 => (15, sprite.tile() & 0xfe),
                    };

                    let sprite_y = match sprite.y_flip() {
                        true  => height - sprite_y,
                        false => sprite_y,
                    };

                    let sprite_x = match sprite.x_flip() {
                        true  => 7 - sprite_x,
                        false => sprite_x,
                    };

                    // Sprites always use TileSet 1
                    let pix = self.pix_color(tile,
                                             sprite_x as u8,
                                             sprite_y as u8,
                                             TileSet::Set1);

                    // White color (0) pre-palette denotes a
                    // transparent pixel
                    if pix != Color::White {
                        // Pixel is not transparent, compute the color
                        // and return that

                        let palette = match sprite.palette() {
                            sprite::Palette::Obp0 => self.obp0,
                            sprite::Palette::Obp1 => self.obp1,
                        };


                        return palette.transform(pix);
                    }
                }
            }
        }

        bg_col.color
    }

}

impl<'a> Show for Gpu<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        try!(write!(f, "Gpu at ({}, {}) [{}] ",
                    self.htick, self.line, self.mode()));

        Ok(())
    }
}

/// All possible color values on the original game boy
#[deriving(PartialEq,Eq,Copy)]
pub enum Color {
    White     = 0,
    LightGrey = 1,
    DarkGrey  = 2,
    Black     = 3,
}

impl Color {
    /// Create a color from a u8 in the range 0...3
    fn from_u8(c: u8) -> Color {
        match c {
            0 => Color::White,
            1 => Color::LightGrey,
            2 => Color::DarkGrey,
            3 => Color::Black,
            _ => panic!("Invalid color: 0x{:02x}", c),
        }
    }
}

/// Palette description
#[deriving(Copy)]
struct Palette {
    /// Each color can be mapped to an other one independently of the
    /// others
    map: [Color, ..4],
}

impl Palette {
    /// Build a palette from a register value.
    ///
    /// Register value is 0bC3C2C1C0 where CX is the output
    /// value for a given value X. So for instance
    /// 0b00_01_10_11 is a palette that reverses the colors.
    fn from_reg(r: u8) -> Palette {
        let mut p = Palette {
            map: [ Color::White,
                   Color::White,
                   Color::White,
                   Color::White, ]
        };

        for i in range(0, p.map.len()) {
            p.map[i] = Color::from_u8((r >> (i * 2)) & 0x3)
        }

        p
    }

    /// Convert palette into register value
    fn into_reg(&self) -> u8 {
        let mut p = 0u8;

        for i in range(0, self.map.len()) {
            p |= (self.map[i] as u8) << (i * 2);
        }

        p
    }

    /// Transform color `c` through the palette
    fn transform(&self, c: Color) -> Color {
        self.map[c as uint]
    }
}

/// Struct used to describe colos that can be transparent
struct AlphaColor {
    /// Pixel color
    color:  Color,
    /// If `true` the color is fully opaque, otherwise fully
    /// transparent.
    opaque: bool,
}

/// There are two tile maps available on the GameBoy. Each map is
/// 32x32x8bits large and contain index values into the tile set for
/// each map.
#[deriving(Copy)]
enum TileMap {
    /// Low map at addresse range [0x9800, 0x9bff]
    Low,
    /// High map at addresse range [0x9c00, 0x9fff]
    High,
}

impl TileMap {
    /// Return tile map base offset in VRAM
    fn base(self) -> u16 {
        match self {
            TileMap::Low  => 0x1800,
            TileMap::High => 0x1c00,
        }
    }
}

/// There are two overlapping tile sets on the Game Boy. Tile sets are
/// 256x16byte large, entries are indexed into the `TileMap`.
#[deriving(Copy)]
enum TileSet {
    /// Tile set #0 in [0x8800, 0x9bff], index is signed [-128, 127]
    Set0,
    /// Tile set #1 in [0x8000, 0x8fff], index is unsigned [0, 255]
    Set1,
}

impl TileSet {
    /// Return VRAM offset of `tile` for the tileset.
    fn tile_addr(self, tile: u8) -> u16 {
        match self {
            // For `Set0` `tile` is signed and in the range [-128,
            // 127]. Tile 0 is at offset 0x1000.
            TileSet::Set0 => (0x1000 + (((tile as i8) as i16) * 16)) as u16,
            // `Set1` is unsigned and starts at offset 0x0000
            TileSet::Set1  => 0x0 + (tile as u16) * 16,
        }
    }
}

/// Sprites can be 8x8 pixels or 8x16 pixels (a pair of 8x8
/// tiles). The setting is global for all sprites.
#[deriving(PartialEq,Eq,Copy)]
enum SpriteSize {
    /// Sprites resolution is 8x8 (i.e. single tile)
    Sz8x8,
    /// Sprites resolution is 8x16 (i.e. two tiles)
    Sz8x16,
}

impl SpriteSize {
    /// Return the height of sprites depending on the SpriteSize
    /// setting
    fn height(self) -> uint {
        match self {
            SpriteSize::Sz8x8  => 8,
            SpriteSize::Sz8x16 => 16,
        }
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

#[cfg(test)]
mod tests {

    /// Make sure the palette conversion to and from register values works
    /// as expected
    #[test]
    fn palette_conversion() {
        for i in range(0u, 0x100) {
            let r = i as u8;

            let p = super::Palette::from_reg(r);
            assert!(p.into_reg() == r);
        }
    }

    /// Make sure color conversion to and from symbolic values works
    #[test]
    fn color_conversion() {
        for v in range(0, 4) {
            let c = super::Color::from_u8(v);

            assert!(c as u8 == v);
        }
    }
}
