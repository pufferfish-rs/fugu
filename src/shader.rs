use super::Context;
use glow::HasContext;
use std::rc::Rc;
use std::str;

pub struct Shader {
    pub(crate) inner: glow::Program,
    pub(crate) uniforms: Vec<UniformInternal>,
    ctx: Rc<glow::Context>,
}

pub struct Uniform {
    pub name: &'static str,
    pub format: UniformFormat,
}

pub(crate) struct UniformInternal {
    pub location: glow::UniformLocation,
    pub format: UniformFormat,
}

#[derive(Clone, Copy, Debug)]
pub enum UniformFormat {
    Float1,
    Float2,
    Float3,
    Float4,
    Int1,
    Int2,
    Int3,
    Int4,
}

impl Shader {
    pub(crate) fn new(ctx: &Context, vert_source: impl AsRef<[u8]>, frag_source: impl AsRef<[u8]>, uniforms: &[Uniform]) -> Self {
        let inner = unsafe {
            let program = ctx.inner.create_program().unwrap();

            let vert = ctx.inner.create_shader(glow::VERTEX_SHADER).unwrap();
            let frag = ctx.inner.create_shader(glow::FRAGMENT_SHADER).unwrap();

            ctx.inner.shader_source(vert, str::from_utf8(vert_source.as_ref()).unwrap());
            ctx.inner.shader_source(frag, str::from_utf8(frag_source.as_ref()).unwrap());

            ctx.inner.compile_shader(vert);
            ctx.inner.compile_shader(frag);

            ctx.inner.attach_shader(program, vert);
            ctx.inner.attach_shader(program, frag);

            ctx.inner.link_program(program);

            ctx.inner.delete_shader(vert);
            ctx.inner.delete_shader(frag);

            ctx.inner.use_program(Some(program));

            program
        };

        let uniforms = uniforms
            .iter()
            .map(|uniform| UniformInternal {
                location: unsafe { ctx.inner.get_uniform_location(inner, uniform.name).unwrap() },
                format: uniform.format,
            })
            .collect();

        Self {
            inner,
            uniforms,
            ctx: ctx.inner.clone(),
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.ctx.delete_program(self.inner);
        }
    }
}
