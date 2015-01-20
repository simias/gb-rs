
// Re-export the public interface defined in sub-modules
pub use ui::sdl2::display::Display;
pub use ui::sdl2::controller::Controller;
pub use ui::sdl2::audio::Audio;

mod display;
mod controller;
mod audio;
