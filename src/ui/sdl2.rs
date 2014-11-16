use sdl2::video::Window;
use sdl2::render::Renderer;
use sdl2::pixels::RGB;
use sdl2::rect::Point;

pub struct Display {
    renderer: Renderer,
}

impl Display {
    pub fn new() -> Display {
        ::sdl2::init(::sdl2::INIT_VIDEO);

        let window = match Window::new("gb-rs",
                                       ::sdl2::video::PosCentered,
                                       ::sdl2::video::PosCentered,
                                       160, 144, ::sdl2::video::OPENGL) {
            Ok(window) => window,
            Err(err)   => panic!("failed to create SDL2 window: {}", err)
        };

        let renderer = match Renderer::from_window(window,
                                                   ::sdl2::render::DriverAuto,
                                                   ::sdl2::render::SOFTWARE) {
            Ok(renderer) => renderer,
            Err(err) => panic!("failed to create SDL2 renderer: {}", err)
        };

        Display { renderer: renderer }
    }
}

impl super::Display for Display {
    fn clear(&mut self) {
        let _ = self.renderer.set_draw_color(RGB(0xff, 0x00, 0x00));
        let _ = self.renderer.clear();
    }

    fn set_pixel(&mut self, x: u32, y: u32, col: u8) {
        let col = match col {
            3     => RGB(0x00, 0x00, 0x00),
            2     => RGB(0x55, 0x55, 0x55),
            1     => RGB(0xab, 0xab, 0xab),
            0     => RGB(0xff, 0xff, 0xff),
            _     => panic!("Unexpected color: {}", col),
        };

        let _ = self.renderer.set_draw_color(col);

        let _ =self.renderer.draw_point(Point::new(x as i32, y as i32));
    }

    fn flip(&mut self) {
        self.renderer.present();
        self.clear();
    }
}
