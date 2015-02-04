use sdl2::video::Window;
use sdl2::render::Renderer;
use sdl2::pixels::Color::RGB;
use sdl2::rect::{Point, Rect};

use gpu::Color;

pub struct Display {
    renderer: Renderer,
    /// Upscaling factor, log2.
    upscale:  u8,
}

impl Display {
    pub fn new(upscale: u8) -> Display {
        ::sdl2::init(::sdl2::INIT_VIDEO);

        let up = 1 << (upscale as usize);

        let xres = 160 * up;
        let yres = 144 * up;

        let window = match Window::new("gb-rs",
                                       ::sdl2::video::WindowPos::PosCentered,
                                       ::sdl2::video::WindowPos::PosCentered,
                                       xres, yres, ::sdl2::video::OPENGL) {
            Ok(window) => window,
            Err(err)   => panic!("failed to create SDL2 window: {}", err)
        };

        let renderer =
            match Renderer::from_window(window,
                                        ::sdl2::render::RenderDriverIndex::Auto,
                                        ::sdl2::render::SOFTWARE) {
            Ok(renderer) => renderer,
            Err(err) => panic!("failed to create SDL2 renderer: {}", err)
        };

        Display { renderer: renderer, upscale: upscale }
    }
}

impl ::ui::Display for Display {
    fn clear(&mut self) {
        let mut drawer = self.renderer.drawer();

        let _ = drawer.set_draw_color(RGB(0xff, 0x00, 0x00));
        let _ = drawer.clear();
    }

    fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let color = match color {
            Color::Black     => RGB(0x00, 0x00, 0x00),
            Color::DarkGrey  => RGB(0x55, 0x55, 0x55),
            Color::LightGrey => RGB(0xab, 0xab, 0xab),
            Color::White     => RGB(0xff, 0xff, 0xff),
        };

        let mut drawer = self.renderer.drawer();

        let _ = drawer.set_draw_color(color);

        if self.upscale == 0 {
            let _ = drawer.draw_point(Point::new(x as i32, y as i32));
        } else {
            let up = 1 << (self.upscale as usize);

            // Translate coordinates
            let x = x as i32 * up;
            let y = y as i32 * up;

            let _ = drawer.fill_rect(Rect::new(x, y, up, up));
        }
    }

    fn flip(&mut self) {
        self.renderer.drawer().present();
        self.clear();
    }
}
