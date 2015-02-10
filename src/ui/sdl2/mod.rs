
use std::cell::Cell;

// Re-export the public interface defined in sub-modules
pub use ui::sdl2::display::Display;
pub use ui::sdl2::controller::Controller;
pub use ui::sdl2::audio::Audio;
pub use ui::sdl2::opengl::OpenGL;

mod display;
mod audio;
mod controller;
mod opengl;

pub struct Context {
    sdl2: ::sdl2::sdl::Sdl,
    controller: controller::Controller,
}

impl Context {
    pub fn new() -> Context {
        let sdl2 =
            ::sdl2::init(::sdl2::INIT_VIDEO |
                         ::sdl2::INIT_GAME_CONTROLLER |
                         ::sdl2::INIT_AUDIO).unwrap();

        Context {
            sdl2: sdl2,
            controller: controller::Controller::new(),
        }
    }

    pub fn new_display(&self, upscale: u8) -> display::Display {
        display::Display::new(&self.sdl2, upscale)
    }

    pub fn opengl_new(&self, xres: u32, yres: u32) -> OpenGL {
        OpenGL::new(&self.sdl2, xres, yres)
    }


    pub fn buttons(&self) -> &Cell<::ui::Buttons> {
        self.controller.buttons()
    }

    pub fn update_buttons(&self) -> ::ui::Event {
        self.controller.update(&self.sdl2)
    }
}
