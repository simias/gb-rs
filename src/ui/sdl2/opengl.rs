use std::ffi::CString;
use std::ptr;
use std::str;
use std::mem;
use libc::c_void;
use gl;
use gl::types::{GLfloat, GLenum, GLuint, GLint, GLchar, GLsizeiptr};
use gl::types::{GLboolean};
use sdl2::video::{Window, WindowPos, GLAttr, OPENGL, GLContext};
use sdl2::video::{gl_set_attribute, gl_get_proc_address};
use sdl2::sdl::Sdl;
use std::iter::repeat;

use gpu::Color;

/// OpenGL-based rendering
pub struct OpenGL {
    /// SDL2 window
    window:  Window,
    /// OpenGL context
    #[allow(dead_code)]
    context: GLContext,
    /// texture representing the GameBoy framebuffer. The screen is
    /// actually smaller than that but I've read that it's better to
    /// use powers of two so why not. We use 24bpp RGB.
    texture: [u8; 256 * 256 * 3],
}

impl OpenGL {
    pub fn new(sdl2: &Sdl, xres: u32, yres: u32) -> OpenGL {
        ::sdl2::init(::sdl2::INIT_VIDEO);

        gl_set_attribute(GLAttr::GLContextMajorVersion, 3);
        gl_set_attribute(GLAttr::GLContextMinorVersion, 3);

        gl_set_attribute(GLAttr::GLDoubleBuffer, 1);
        gl_set_attribute(GLAttr::GLDepthSize, 24);

        let window = match Window::new(sdl2,
                                       "gb-rs",
                                       WindowPos::PosCentered,
                                       WindowPos::PosCentered,
                                       xres as i32, yres as i32,
                                       OPENGL) {
            Ok(window) => window,
            Err(err)   => panic!("failed to create SDL2 window: {}", err)
        };

        let context = match window.gl_create_context() {
            Ok(context) => context,
            Err(err)    => panic!("failed to create OpenGL context: {}", err),
        };

        // Load OpenGL function pointers from SDL2
        ::gl::load_with(|s| {
            match gl_get_proc_address(s) {
                Some(p) => p as *const c_void,
                None    => panic!("can't get proc address for {}", s),
            }
        });

        let vertex_shader =
            compile_shader(
                "#version 330 core                                \n\
                                                                  \n\
                 in  vec2 position;                               \n\
                 in  vec2 vertex_uv;                              \n\
                                                                  \n\
                 out vec2 uv;                                     \n\
                                                                  \n\
                 void main(void) {                                \n\
                     gl_Position.xyzw = vec4(position, 0.0, 1.0); \n\
                     uv = vertex_uv;                              \n\
                 }",
                gl::VERTEX_SHADER);

        let fragment_shader =
            compile_shader(
                "#version 330 core                                \n\
                                                                  \n\
                 in  vec2 uv;                                     \n\
                                                                  \n\
                 out vec3 color;                                  \n\
                                                                  \n\
                 uniform sampler2D gb_screen;                     \n\
                                                                  \n\
                 void main(void) {                                \n\
                     color = texture2D(gb_screen, uv).rgb;        \n\
                 }",
                gl::FRAGMENT_SHADER);

        let program = link_program(vertex_shader, fragment_shader);

        let vertices: [GLfloat; 12] = [
            -1., -1.,
            -1.,  1.,
             1., -1.,

             1.,  1.,
            -1.,  1.,
             1., -1.,
            ];

        // We crop the texture to the actual screen resolution
        let u_max = 160. / 255.;
        let v_max = 144. / 255.;

        let uv_mapping: [GLfloat; 12] = [
             0.,    v_max,
             0.,    0.,
             u_max, v_max,

             u_max, 0.,
             0.,    0.,
             u_max, v_max,
            ];

        let mut vertex_array_object  = 0;
        let mut vertex_buffer_object = 0;
        let mut uv_buffer_object = 0;
        let mut texture = 0;
        let mut texture_id;

        unsafe {
            gl::GenVertexArrays(1, &mut vertex_array_object);
            gl::BindVertexArray(vertex_array_object);

            // Generate vertex buffer
            gl::GenBuffers(1, &mut vertex_buffer_object);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_object);

            let pos_attr = gl::GetAttribLocation(program,
                                                 CString::new("position").unwrap().as_ptr());

            gl::EnableVertexAttribArray(pos_attr as GLuint);
            gl::VertexAttribPointer(pos_attr as GLuint, 2, gl::FLOAT,
                                    gl::FALSE as GLboolean, 0, ptr::null());
            
            gl::BufferData(gl::ARRAY_BUFFER,
                           (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                           mem::transmute(&vertices[0]),
                           gl::STATIC_DRAW);

            // Generate uv buffer
            gl::GenBuffers(1, &mut uv_buffer_object);
            gl::BindBuffer(gl::ARRAY_BUFFER, uv_buffer_object);

            let pos_attr = gl::GetAttribLocation(program,
                                                 CString::new("vertex_uv").unwrap().as_ptr());

            gl::EnableVertexAttribArray(pos_attr as GLuint);
            gl::VertexAttribPointer(pos_attr as GLuint, 2, gl::FLOAT,
                                    gl::FALSE as GLboolean, 0, ptr::null());

            
            gl::BufferData(gl::ARRAY_BUFFER,
                           (uv_mapping.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                           mem::transmute(&uv_mapping[0]),
                           gl::STATIC_DRAW);

            // Create the texture used to render the GB screen
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            gl::TexParameteri(gl::TEXTURE_2D,
                              gl::TEXTURE_MAG_FILTER,
                              gl::NEAREST as GLint);

            gl::TexParameteri(gl::TEXTURE_2D,
                              gl::TEXTURE_MIN_FILTER,
                              gl::NEAREST as GLint);

            texture_id = gl::GetUniformLocation(program,
                                                CString::new("gb_screen").unwrap().as_ptr());

            gl::Uniform1i(texture_id, texture as GLint);


            // Use shader program
            gl::UseProgram(program);

            gl::BindFragDataLocation(program, 0,
                                     CString::new("color").unwrap().as_ptr());

            gl::ClearColor(0., 0., 0., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        OpenGL {
            window:  window,
            context: context,
            texture: [0; 256 * 256 * 3],
        }
    }
}

impl ::ui::Display for OpenGL {

    fn clear(&mut self) {
        self.texture = [0; 256 * 256 * 3];
    }

    fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let color = match color {
            Color::Black     => [0x00, 0x00, 0x00],
            Color::DarkGrey  => [0x55, 0x55, 0x55],
            Color::LightGrey => [0xab, 0xab, 0xab],
            Color::White     => [0xff, 0xff, 0xff],
        };

        let pos = y * (256 * 3) + x * 3;
        let pos = pos as usize;

        self.texture[pos + 0] = color[0];
        self.texture[pos + 1] = color[1];
        self.texture[pos + 2] = color[2];
    }

    fn flip(&mut self) {
        unsafe {
            gl::TexImage2D(gl::TEXTURE_2D,
                           0,
                           gl::RGB as GLint,
                           256, 256,
                           0,
                           gl::RGB,
                           gl::UNSIGNED_BYTE,
                           mem::transmute(&self.texture[0]));

            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }

        self.window.gl_swap_window();
        self.clear();
    }
}

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        // Attempt to compile the shader
        let c_str = CString::new(src).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            // subtract 1 to skip the trailing null character
            let mut buf: Vec<_> = repeat(0).take(len as usize - 1).collect();
            gl::GetShaderInfoLog(shader,
                                 len, ptr::null_mut(),
                                 buf.as_mut_ptr() as *mut GLchar);
            panic!("{}",
                   str::from_utf8(&buf).ok()
                   .expect("ShaderInfoLog not valid utf8"));
        }
    }
    shader
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf: Vec<_> = repeat(0).take(len as usize - 1).collect();
            gl::GetProgramInfoLog(program,
                                  len,
                                  ptr::null_mut(),
                                  buf.as_mut_ptr() as *mut GLchar);
            panic!("{}",
                   str::from_utf8(&buf).ok()
                   .expect("ProgramInfoLog not valid utf8"));
        }
        program
    }
}
