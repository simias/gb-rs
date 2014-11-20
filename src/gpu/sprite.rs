//! Game Boy GPU Sprite emulation

/// Sprite metadata
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
            palette:    Palette::Obj0,
        }
    }

    pub fn set_x_pos(&mut self, x: u8) {
        self.x = x;
    }

    pub fn x_pos(&self) -> u8 {
        self.x
    }

    pub fn set_y_pos(&mut self, y: u8) {
        self.y = y;
    }

    pub fn y_pos(&self) -> u8 {
        self.y
    }

    pub fn set_tile(&mut self, tile: u8) {
        self.tile = tile;
    }

    pub fn tile(&self) -> u8 {
        self.tile
    }

    /// Set sprite miscellaneous flags. `flags` is the OAM memory byte
    /// representing the flags.
    pub fn set_flags(&mut self, flags: u8) {
        self.background = flags & 0x80 != 0;
        self.y_flip     = flags & 0x40 != 0;
        self.x_flip     = flags & 0x20 != 0;
        self.palette    = match flags & 0x10 != 0 {
            false => Palette::Obj0,
            true  => Palette::Obj1,
        }
    }

    /// Reconstruct flags register value from Sprite state
    pub fn flags(&self) -> u8 {
        let mut r = 0;

        r |= (self.background as u8) << 7;
        r |= (self.y_flip     as u8) << 6;
        r |= (self.x_flip     as u8) << 5;

        r |= match self.palette {
            Palette::Obj0 => 0,
            Palette::Obj1 => 1,
        } << 4;

        r
    }
}

/// Sprites can use two palettes
enum Palette {
    /// Pallette OBJ0
    Obj0,
    /// Pallette OBJ1
    Obj1,
}
