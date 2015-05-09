//! Game Boy GPU Sprite emulation

/// Sprite metadata
#[derive(Clone,Copy)]
pub struct Sprite {
    /// Top left corner x-coordinate + 8
    x: u8,
    /// Top left corner y-coordinate + 16
    y: u8,
    /// Number of the tile containing the actual sprite pattern
    tile: u8,
    /// If `true` sprite is displayed behind the background and
    /// window, otherwise it's on top.
    background: bool,
    /// If `true` the tile pattern is flipped horizontally
    x_flip: bool,
    /// If `true` the tile pattern is flipped vertically
    y_flip: bool,
    /// Which palette the sprite uses
    palette: Palette,
}

impl Sprite {
    pub fn new() -> Sprite {
        Sprite {
            x:          0,
            y:          0,
            tile:       0,
            background: false,
            x_flip:     false,
            y_flip:     false,
            palette:    Palette::Obp0,
        }
    }

    pub fn set_x_pos(&mut self, x: u8) {
        self.x = x;
    }

    /// Return the sprite's x position as configured in the
    /// register. The actual position of the top left corner on the
    /// screen is `x_pos - 8`
    pub fn x_pos(&self) -> u8 {
        self.x
    }

    /// Return the sprite's y position as configured in the
    /// register. The actual position of the top left corner on the
    /// screen is `y_pos - 16`
    pub fn set_y_pos(&mut self, y: u8) {
        self.y = y;
    }

    pub fn y_pos(&self) -> u8 {
        self.y
    }

    /// Return the number of top line of the sprite in real screen
    /// coordinates.
    pub fn top_line(&self) -> i32 {
        (self.y as i32) - 16
    }

    /// Return the number of the left line of the sprite in real
    /// screen coordinates.
    pub fn left_column(&self) -> i32 {
        (self.x as i32) - 8
    }

    pub fn set_tile(&mut self, tile: u8) {
        self.tile = tile;
    }

    pub fn tile(&self) -> u8 {
        self.tile
    }

    pub fn background(&self) -> bool {
        self.background
    }

    pub fn palette(&self) -> Palette {
        self.palette
    }

    pub fn x_flip(&self) -> bool {
        self.x_flip
    }

    pub fn y_flip(&self) -> bool {
        self.y_flip
    }

    /// Set sprite miscellaneous flags. `flags` is the OAM memory byte
    /// representing the flags.
    pub fn set_flags(&mut self, flags: u8) {
        self.background = flags & 0x80 != 0;
        self.y_flip     = flags & 0x40 != 0;
        self.x_flip     = flags & 0x20 != 0;
        self.palette    = match flags & 0x10 != 0 {
            false => Palette::Obp0,
            true  => Palette::Obp1,
        }
    }

    /// Reconstruct flags register value from Sprite state
    pub fn flags(&self) -> u8 {
        let mut r = 0;

        r |= (self.background as u8) << 7;
        r |= (self.y_flip     as u8) << 6;
        r |= (self.x_flip     as u8) << 5;

        r |= match self.palette {
            Palette::Obp0 => 0,
            Palette::Obp1 => 1,
        } << 4;

        r
    }
}

/// Sprites can use two palettes
#[derive(Clone,Copy)]
pub enum Palette {
    /// Pallette OBP0
    Obp0,
    /// Pallette OBP1
    Obp1,
}
