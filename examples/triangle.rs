use fugu::*;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

#[repr(C)]
struct Vertex {
    pos: (f32, f32),
    color: (f32, f32, f32),
}

fn main() {
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("Hello Triangle");

    let windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    let ctx = Context::new(|s| windowed_context.context().get_proc_address(s));

    let frag_source = r"
        #version 330

        in vec3 vert_color;

        out vec4 out_color;

        void main() {
            out_color = vec4(vert_color, 1.0);
        }
    ";

    let vert_source = r"
        #version 330

        in vec2 pos;
        in vec3 color;

        out vec3 vert_color;

        void main() {
            gl_Position = vec4(pos, 0.0, 1.0);
            vert_color = color;
        }
    ";

    let shader = ctx.create_shader(vert_source, frag_source, &[], &[]);

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
        ],
    );

    let verts = &[
        Vertex {
            pos: (0.0, 0.5),
            color: (1., 0., 0.),
        },
        Vertex {
            pos: (0.5, -0.5),
            color: (0., 1., 0.),
        },
        Vertex {
            pos: (-0.5, -0.5),
            color: (0., 0., 1.),
        },
    ];

    let buffer = ctx.create_buffer_with_data(BufferKind::Vertex, BufferUsage::Static, verts);

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

                ctx.set_pipeline(&pipeline);
                ctx.set_vertex_buffer(&buffer);

                ctx.draw(0, 3, 1);

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
