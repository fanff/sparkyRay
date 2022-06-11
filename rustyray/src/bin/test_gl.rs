use crate::render_gl::{create_whitespace_cstring_with_len, Shader};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::os::raw::c_char;
use std::time::{Duration, Instant};

extern crate core;
extern crate gl;

pub mod render_gl {
    use gl;
    use std;
    use std::ffi::{CStr, CString};

    pub struct Shader {
        id: gl::types::GLuint,
    }

    impl Shader {
        pub fn from_source(source: &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
            let id = shader_from_source(source, kind)?;
            Ok(Shader { id })
        }

        pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
            Shader::from_source(source, gl::VERTEX_SHADER)
        }

        pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
            Shader::from_source(source, gl::FRAGMENT_SHADER)
        }
        pub fn id(&self) -> gl::types::GLuint {
            self.id
        }
    }

    impl Drop for Shader {
        fn drop(&mut self) {
            unsafe {
                gl::DeleteShader(self.id);
            }
        }
    }

    fn shader_from_source(
        source: &CStr,
        kind: gl::types::GLenum,
    ) -> Result<gl::types::GLuint, String> {
        let id = unsafe { gl::CreateShader(kind) };
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetShaderInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        Ok(id)
    }

    pub fn create_whitespace_cstring_with_len(len: usize) -> CString {
        // allocate buffer of correct size
        let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
        // fill it with len spaces
        buffer.extend([b' '].iter().cycle().take(len));
        // convert buffer to CString
        unsafe { CString::from_vec_unchecked(buffer) }
    }

    pub struct Program {
        id: gl::types::GLuint,
    }

    impl Program {
        pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
            let program_id = unsafe { gl::CreateProgram() };

            for shader in shaders {
                unsafe {
                    gl::AttachShader(program_id, shader.id());
                }
            }

            unsafe {
                gl::LinkProgram(program_id);
                let mut success: gl::types::GLint = 1;
                unsafe {
                    gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
                }

                if success == 0 {
                    let mut len: gl::types::GLint = 0;
                    unsafe {
                        gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
                    }

                    let error = create_whitespace_cstring_with_len(len as usize);

                    unsafe {
                        gl::GetProgramInfoLog(
                            program_id,
                            len,
                            std::ptr::null_mut(),
                            error.as_ptr() as *mut gl::types::GLchar,
                        );
                    }

                    return Err(error.to_string_lossy().into_owned());
                }
            }

            // continue with error handling here

            for shader in shaders {
                unsafe {
                    gl::DetachShader(program_id, shader.id());
                }
            }

            Ok(Program { id: program_id })
        }

        pub fn id(&self) -> gl::types::GLuint {
            self.id
        }
        pub fn set_used(&self) {
            unsafe {
                gl::UseProgram(self.id);
            }
        }
    }

    impl Drop for Program {
        fn drop(&mut self) {
            unsafe {
                gl::DeleteProgram(self.id);
            }
        }
    }
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    //gl_attr.set_context_version(4, 6);

    let window = video_subsystem
        .window("Game", 300, 300)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let _gl_context = window.gl_create_context().unwrap();
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::Viewport(0, 0, 300, 300);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    use std::ffi::CString;
    let vert_shader = render_gl::Shader::from_vert_source(
        &CString::new(include_str!("../../test.vert")).unwrap(),
    )
    .unwrap();
    let frag_shader = render_gl::Shader::from_frag_source(
        &CString::new(include_str!("../../triangle_collide.frag")).unwrap(),
    )
    .unwrap();

    let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    // set up vertex buffer object

    let vertices: Vec<f32> = vec![
        // positions      // colors
        -1.0, -1.0, 0.0, // bottom left
        -1.0, 1.0, 0.0, // top left
        1.0, 1.0, 0.0, // top right
        1.0, -1.0, 0.0, // bottom right
    ];
    // let trianglesSet: Vec<f32> = vec![0.0; 1024 * 3 * 3];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,                                                       // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW,                               // usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    // set up vertex array object
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }

    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            0,         // index of the generic vertex attribute ("layout (location = 0)")
            3,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null(),                                     // offset of the first component
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
    const TCOUNT: u32 = 1024 * 3;
    const LIGHT_COUNT: u32 = 16;

    //let vertices: Vec<f32> = vec![0.0; TCOUNT as usize];
    //
    //let mut myArrayUBO: gl::types::GLuint = 0;
    //unsafe {
    //    gl::GenBuffers(1, &mut myArrayUBO);
    //}
    //unsafe {
    //    // Allocate storage for the UBO
    //    gl::BindBuffer(gl::UNIFORM_BUFFER, myArrayUBO);
    //    gl::BufferData(
    //        gl::UNIFORM_BUFFER,
    //        (std::mem::size_of::<gl::types::GLfloat>() * TCOUNT as usize) as gl::types::GLsizeiptr,
    //        vertices.as_ptr() as *const gl::types::GLvoid,
    //        gl::DYNAMIC_DRAW,
    //    );
    //}
    //
    //unsafe {
    //    //GLuint
    //
    //    let c_str = CString::new("myArrayBlock").unwrap();
    //    let c_world: *const c_char = c_str.as_ptr() as *const c_char;
    //    let myArrayBlockIdx = gl::GetUniformBlockIndex(shader_program.id(), c_world);
    //
    //    gl::UniformBlockBinding(shader_program.id(), myArrayBlockIdx, 0);
    //    gl::BindBufferBase(gl::UNIFORM_BUFFER, 0, myArrayUBO);
    //}

    let cameraData: [f32; 12] = [0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0];

    //let cameraData = [
    //    [0.0, 0.0, 0.0, 1.0],
    //    [0.0, 0.0, 1.0, 0.0],
    //    [0.0, 1.0, 0.0, 0.0],
    //    [1.0, 0.0, 0.0, 0.0],
    //];

    // camera  settings
    // layout (std140) uniform cameraBlock {
    //     vec3 cam_origin;
    //     vec3 orthx  ;
    //     vec3 orthy  ;
    //     vec3 cam_direction;
    // };

    let mut cameraData_bo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut cameraData_bo);
    }
    unsafe {
        gl::BindBuffer(gl::UNIFORM_BUFFER, cameraData_bo);
        gl::BufferData(
            gl::UNIFORM_BUFFER,
            (std::mem::size_of::<gl::types::GLfloat>() * 12) as gl::types::GLsizeiptr,
            std::ptr::null(), //cameraData.as_ptr() as *const gl::types::GLvoid,
            gl::DYNAMIC_DRAW,
        );
    }

    unsafe {
        //GLuint

        let c_str = CString::new("cameraBlock").unwrap();
        let c_world: *const c_char = c_str.as_ptr() as *const c_char;
        let cameraData_idx = gl::GetUniformBlockIndex(shader_program.id(), c_world);

        gl::UniformBlockBinding(shader_program.id(), cameraData_idx, 0);
        gl::BindBufferBase(gl::UNIFORM_BUFFER, 0, cameraData_bo);

        //void *glMapBufferRange(GLenum target, GLintptr offset, GLsizeiptr , GLbitfield access);
    }

    //vec3 cam_origin;
    //vec3 orthx  ;
    //vec3 orthy  ;
    //vec3 direction;
    //let mouse = sdl_context.mouse();
    //
    //let mut canvas = window
    //    .into_canvas()
    //    .build()
    //    .map_err(|e| e.to_string())
    //    .unwrap();
    //canvas.clear();
    //
    //let texture_creator = canvas.texture_creator();
    //
    //const texture_w: u32 = 16;
    //const texture_h: u32 = 12;
    //
    let mut strt_loop_time;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                //Event::MouseMotion { xrel, yrel, .. } => {
                //    // println!("{:?}", event)
                //    mousex += xrel;
                //    mousey += yrel;
                //}
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    println!("S");
                    unsafe {
                        gl::Clear(gl::COLOR_BUFFER_BIT);
                        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
                    }
                }
                _ => {}
            }
        }
        strt_loop_time = Instant::now();
        shader_program.set_used();
        unsafe {
            //gl::PointSize(1.0);
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLE_FAN, // mode
                0,                // starting index in the enabled arrays
                4,                // number of indices to be rendered
            );
        }

        window.gl_swap_window();

        let loop_dur = (Instant::now() - strt_loop_time).as_millis();
        println!("swap ! {:?} ", loop_dur);
        ::std::thread::sleep(Duration::from_millis(10));
    }
}

//http://nercury.github.io/rust/opengl/tutorial/2018/02/11/opengl-in-rust-from-scratch-04-triangle.html
