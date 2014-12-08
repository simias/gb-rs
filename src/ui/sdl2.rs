use sdl2::video::Window;
use sdl2::render::Renderer;
use sdl2::pixels::Color::RGB;
use sdl2::rect::{Point, Rect};

use sdl2::event::Event;
use sdl2::keycode::KeyCode;

use super::{ButtonState};

pub struct Display {
    renderer: Renderer,
    /// Upscaling factor, log2.
    upscale:  uint,
}

impl Display {
    pub fn new(upscale: uint) -> Display {
        ::sdl2::init(::sdl2::INIT_VIDEO);

        let up = 1 << upscale;

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

        if self.upscale == 0 {
            let _ = self.renderer.draw_point(Point::new(x as i32, y as i32));
        } else {
            let up = 1 << self.upscale;

            // Translate coordinates
            let x = x as i32 * up;
            let y = y as i32 * up;

            let _ = self.renderer.fill_rect(&Rect::new(x, y, up, up));
        }
    }

    fn flip(&mut self) {
        self.renderer.present();
        self.clear();
    }
}

pub struct Controller {
    buttons: super::Buttons,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            buttons: super::Buttons::new(ButtonState::Up),
        }
    }

    /// Update key state. For now keybindings are hardcoded.
    fn update_key(&mut self, key: KeyCode, state: ButtonState) {
         match key {
             KeyCode::Up        => self.buttons.up     = state,
             KeyCode::Down      => self.buttons.down   = state,
             KeyCode::Left      => self.buttons.left   = state,
             KeyCode::Right     => self.buttons.right  = state,
             KeyCode::LCtrl     => self.buttons.a      = state,
             KeyCode::LAlt      => self.buttons.b      = state,
             KeyCode::Return    => self.buttons.start  = state,
             KeyCode::Backspace => self.buttons.select = state,
             _                  => (),
         }
    }
}

impl super::Controller for Controller {
    fn update(&mut self) -> super::Event {
        let mut event = super::Event::None;

        loop {
            match ::sdl2::event::poll_event() {
                Event::None =>
                    break,
                Event::KeyDown(_, _, KeyCode::Escape, _, _, _) =>
                    event = super::Event::PowerOff,
                Event::KeyDown(_, _, key, _, _, _) =>
                    self.update_key(key, ButtonState::Down),
                Event::KeyUp(_, _, key, _, _, _) =>
                    self.update_key(key, ButtonState::Up),
                _ => ()
            }
        }

        event
    }

    fn state(&self) -> super::Buttons {
        self.buttons
    }
}
