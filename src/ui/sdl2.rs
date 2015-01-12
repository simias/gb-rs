use std::cell::Cell;

use sdl2::video::Window;
use sdl2::render::Renderer;
use sdl2::pixels::Color::RGB;
use sdl2::rect::{Point, Rect};

use sdl2::event::Event;
use sdl2::keycode::KeyCode;

use super::{ButtonState};
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

impl super::Display for Display {
    fn clear(&mut self) {
        let _ = self.renderer.set_draw_color(RGB(0xff, 0x00, 0x00));
        let _ = self.renderer.clear();
    }

    fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let color = match color {
            Color::Black     => RGB(0x00, 0x00, 0x00),
            Color::DarkGrey  => RGB(0x55, 0x55, 0x55),
            Color::LightGrey => RGB(0xab, 0xab, 0xab),
            Color::White     => RGB(0xff, 0xff, 0xff),
        };

        let _ = self.renderer.set_draw_color(color);

        if self.upscale == 0 {
            let _ = self.renderer.draw_point(Point::new(x as i32, y as i32));
        } else {
            let up = 1 << (self.upscale as usize);

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
    buttons: Cell<super::Buttons>,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            buttons: Cell::new(super::Buttons::new(ButtonState::Up)),
        }
    }

    /// Update key state. For now keybindings are hardcoded.
    fn update_key(&self, key: KeyCode, state: ButtonState) {
        let mut b = self.buttons.get();

        match key {
            KeyCode::Up        => b.up     = state,
            KeyCode::Down      => b.down   = state,
            KeyCode::Left      => b.left   = state,
            KeyCode::Right     => b.right  = state,
            KeyCode::LAlt      => b.a      = state,
            KeyCode::LCtrl     => b.b      = state,
            KeyCode::Return    => b.start  = state,
            KeyCode::RShift    => b.select = state,
            _                  => (),
        }

        self.buttons.set(b);
    }
}

impl super::Controller for Controller {
    fn update(&self) -> super::Event {
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
                Event::Quit(_) =>
                    event = super::Event::PowerOff,
                _ => ()
            }
        }

        event
    }

    fn buttons(&self) -> &Cell<super::Buttons> {
        &self.buttons
    }
}
