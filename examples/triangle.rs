use fugu::*;
use image::EncodableLayout;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Instant;

#[repr(C)]
struct Vertex {
    pos: (f32, f32),
    color: (f32, f32, f32),
    tex_coord: (f32, f32),
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Hello triangle", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_version(3, 3);
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);

    let _gl_context = window.gl_create_context()?;

    let mut event_pump = sdl_context.event_pump()?;

    let ctx = Context::new(|s| video_subsystem.gl_get_proc_address(s).cast());

    ctx.set_viewport(0, 0, 800, 600);

    let frag_source = r"
        #version 330

        uniform sampler2D tex;

        in vec3 vert_color;
        in vec2 vert_tex_coord;

        out vec4 out_color;

        void main() {
            out_color = vec4(vert_color, 1.0) * texture(tex, vert_tex_coord);
        }
    ";

    let vert_source = r"
        #version 330

        uniform float time;

        in vec2 pos;
        in vec3 color;
        in vec2 tex_coord;

        out vec3 vert_color;
        out vec2 vert_tex_coord;

        void main() {
            gl_Position = vec4(pos.x + sin(time * 2.5) * 0.2, pos.y + sin(time * 5.0) * 0.1, 0.0, 1.0);
            vert_color = color;
            vert_tex_coord = tex_coord;
        }
    ";

    let shader = ctx.create_shader(
        vert_source,
        frag_source,
        &[Uniform {
            name: "time",
            format: UniformFormat::Float1,
        }],
        &[ImageUniform { name: "tex" }],
    );

    let pipeline = ctx.create_pipeline(
        shader,
        &[BufferLayout::default()],
        &[
            VertexAttribute {
                name: "pos",
                format: VertexFormat::Float2,
                buffer_index: 0,
            },
            VertexAttribute {
                name: "color",
                format: VertexFormat::Float3,
                buffer_index: 0,
            },
            VertexAttribute {
                name: "tex_coord",
                format: VertexFormat::Float2,
                buffer_index: 0,
            },
        ],
    );

    let verts = &[
        Vertex {
            pos: (-0.5, -0.5),
            color: (1., 0., 0.),
            tex_coord: (0., 0.),
        },
        Vertex {
            pos: (0.5, -0.5),
            color: (0., 1., 0.),
            tex_coord: (1., 0.),
        },
        Vertex {
            pos: (0.5, 0.5),
            color: (0., 0., 1.),
            tex_coord: (1., 1.),
        },
    ];

    let buffer = ctx.create_buffer_with_data(BufferKind::Vertex, BufferUsage::Static, verts);

    let pattern_tex = {
        let image = image::load_from_memory(include_bytes!("pattern.png")).unwrap();
        let data = image.to_rgba8();
        let (width, height) = data.dimensions();
        ctx.create_image_with_data(width, height, ImageFormat::Rgba8, ImageFilter::Linear, ImageWrap::Repeat, data.as_bytes())
    };

    let start_time = Instant::now();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        ctx.begin_default_pass(PassAction::Clear {
            color: Some((0., 0., 0., 1.)),
            depth: None,
            stencil: None,
        });

        ctx.set_pipeline(&pipeline);
        ctx.set_vertex_buffer(&buffer);
        ctx.set_uniforms(start_time.elapsed().as_secs_f32());
        ctx.set_images(&[&pattern_tex]);

        ctx.draw(0, 3, 1);

        ctx.end_render_pass();
        ctx.commit_frame();

        window.gl_swap_window();
    }

    Ok(())
}
