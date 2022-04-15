use std::time::Instant;

use fugu::*;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use image::EncodableLayout;

#[repr(C)]
struct Vertex {
    pos: (f32, f32),
    color: (f32, f32, f32),
    tex_coord: (f32, f32),
}

fn main() {
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("Textured quad");

    let windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    let ctx = Context::new(|s| windowed_context.context().get_proc_address(s));

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
            gl_Position = vec4(pos, 0.0, 1.0);
            vert_color = color;
            vert_tex_coord = tex_coord + vec2(sin(time), cos(time));
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
            pos: (-0.5, 0.5),
            color: (1., 0., 0.),
            tex_coord: (0., 1.),
        },
        Vertex {
            pos: (0.5, 0.5),
            color: (0., 1., 0.),
            tex_coord: (1., 1.),
        },
        Vertex {
            pos: (0.5, -0.5),
            color: (0., 0., 1.),
            tex_coord: (1., 0.),
        },
        Vertex {
            pos: (-0.5, -0.5),
            color: (1., 1., 0.),
            tex_coord: (0., 0.),
        },
    ];
    let idx: &[u16] = &[0, 1, 2, 0, 2, 3];

    let vert_buffer = ctx.create_buffer_with_data(BufferKind::Vertex, BufferUsage::Static, verts);
    let idx_buffer = ctx.create_buffer_with_data(BufferKind::Index, BufferUsage::Static, idx);

    let pattern_tex = {
        let image = image::load_from_memory(include_bytes!("pattern.png")).unwrap();
        let data = image.to_rgba8();
        let (width, height) = data.dimensions();
        ctx.create_image_with_data(
            width,
            height,
            ImageFormat::Rgba8,
            ImageFilter::Linear,
            ImageWrap::Repeat,
            data.as_bytes(),
        )
    };

    let start_time = Instant::now();

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    ctx.set_viewport(0, 0, physical_size.width, physical_size.height);
                    windowed_context.resize(physical_size)
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {
                ctx.begin_default_pass(PassAction::Clear {
                    color: Some((0., 0., 0., 1.)),
                    depth: None,
                    stencil: None,
                });

                ctx.begin_default_pass(PassAction::Clear {
                    color: Some((0., 0., 0., 1.)),
                    depth: None,
                    stencil: None,
                });

                ctx.set_pipeline(&pipeline);
                ctx.set_vertex_buffer(&vert_buffer);
                ctx.set_index_buffer(&idx_buffer);
                ctx.set_uniforms(start_time.elapsed().as_secs_f32());
                ctx.set_images(&[&pattern_tex]);

                ctx.draw(0, 6, 1);

                ctx.end_render_pass();
                ctx.commit_frame();

                windowed_context.swap_buffers().unwrap();
            }
            Event::MainEventsCleared => {
                windowed_context.window().request_redraw();
            }
            _ => (),
        }
    });
}
