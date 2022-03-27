use rogl::gl;
use rogl::gl::context::GLContext;
use rogl::gl::enums;
use rogl::gl::gl45::GL45;
use std::ffi::CStr;
use std::ptr;

use rogl::types::{GLint, GLsizeiptr, GLuint};

fn main() {
    unsafe {
        // Create a context from a sdl2 window
        let (cntx, window, mut events_loop, _context) = create_sdl2_context();

        let vertex_shader_source: &[u8] = b"#version 130
          in vec2 in_position;
          out vec2 position;
          void main() {
            position = in_position;
            gl_Position = vec4(in_position - 0.5, 0.0, 1.0);
          }\0";
        let fragment_shader_source: &[u8] = b"#version 130
          precision mediump float;
          in vec2 position;
          out vec4 color;
          uniform float blue;
          void main() {
            color = vec4(position, blue, 1.0);
          }\0";

        // Create a shader program from source
        let program = create_program(&cntx, vertex_shader_source, fragment_shader_source);

        // gl.use_program(Some(program));
        GL45::glUseProgram(&cntx, program);

        // Create a vertex buffer and vertex array object
        let (vbo, vao) = create_vertex_buffer(&cntx);

        // Upload some uniforms
        set_uniform(
            &cntx,
            program,
            CStr::from_bytes_with_nul_unchecked(b"blue\0"),
            0.8,
        );

        GL45::glClearColor(&cntx, 0.1, 0.2, 0.3, 1.0);

        'render: loop {
            {
                for event in events_loop.poll_iter() {
                    if let sdl2::event::Event::Quit { .. } = event {
                        break 'render;
                    }
                }
            }

            GL45::glClear(&cntx, rogl::gl::bitflags::GL_COLOR_BUFFER_BIT);
            GL45::glDrawArrays(&cntx, enums::GL_TRIANGLES, 0, 3);
            window.gl_swap_window();
        }

        // Clean up
        GL45::glDeleteProgram(&cntx, program);
        GL45::glDeleteVertexArrays(&cntx, 1, [vao].as_ptr());
        GL45::glDeleteBuffers(&cntx, 1, [vbo].as_ptr());
    }
}

unsafe fn create_sdl2_context() -> (
    GLContext,
    sdl2::video::Window,
    sdl2::EventPump,
    sdl2::video::GLContext,
) {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 0);
    let window = video
        .window("Hello triangle!", 1024, 769)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    let gl_context = window.gl_create_context().unwrap();
    let context = GLContext::load(|s| {
        video.gl_get_proc_address(s.to_str().expect("failed to conver string")) as *const _
    });
    let event_loop = sdl.event_pump().unwrap();

    (context, window, event_loop, gl_context)
}

unsafe fn create_program<T>(
    cntx: &T,
    vertex_shader_source: &[u8],
    fragment_shader_source: &[u8],
) -> GLuint
where
    T: GL45,
{
    let program = cntx.glCreateProgram();

    let shader_sources = [
        (gl::enums::GL_VERTEX_SHADER, vertex_shader_source),
        (gl::enums::GL_FRAGMENT_SHADER, fragment_shader_source),
    ];

    let mut shaders = Vec::with_capacity(shader_sources.len());

    for (shader_type, shader_source) in shader_sources.iter() {
        let shader = cntx.glCreateShader(*shader_type);
        cntx.glShaderSource(
            shader,
            1,
            &(shader_source.as_ptr() as *const _),
            &(shader_source.len() as GLint),
        );
        cntx.glCompileShader(shader);

        cntx.glAttachShader(program, shader);
        shaders.push(shader);
    }

    cntx.glLinkProgram(program);

    for shader in shaders {
        cntx.glDetachShader(program, shader);
        cntx.glDeleteShader(shader);
    }

    program
}

unsafe fn create_vertex_buffer<T>(ctx: &T) -> (GLuint, GLuint)
where
    T: GL45,
{
    // This is a flat array of f32s that are to be interpreted as vec2s.
    let triangle_vertices = [0.5f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32];
    let triangle_vertices_u8: &[u8] = core::slice::from_raw_parts(
        triangle_vertices.as_ptr() as *const u8,
        triangle_vertices.len() * core::mem::size_of::<f32>(),
    );

    // We construct a buffer and upload the data
    let mut vbo_buffers: [GLuint; 1] = [0; 1];
    ctx.glCreateBuffers(1, vbo_buffers.as_mut_ptr());
    ctx.glBindBuffer(enums::GL_ARRAY_BUFFER, vbo_buffers[0]);
    ctx.glBufferData(
        enums::GL_ARRAY_BUFFER,
        triangle_vertices_u8.len() as GLsizeiptr,
        triangle_vertices_u8.as_ptr() as _,
        enums::GL_STATIC_DRAW,
    );

    // We now construct a vertex array to describe the format of the input buffer
    let mut vao_buffers: [GLuint; 1] = [0; 1];
    ctx.glCreateVertexArrays(1, vao_buffers.as_mut_ptr());
    ctx.glBindVertexArray(vao_buffers[0]);
    ctx.glEnableVertexAttribArray(0);
    ctx.glVertexAttribPointer(0, 2, enums::GL_FLOAT, 0, 8, ptr::null());

    (vbo_buffers[0], vao_buffers[0])
}

unsafe fn set_uniform<T>(gl: &T, program: GLuint, name: &CStr, value: f32)
where
    T: GL45,
{
    let uniform_location = gl.glGetUniformLocation(program, name.as_ptr());
    gl.glUniform1f(uniform_location, value);
}
