use super::Context;
use glow::HasContext;
use std::rc::Rc;
use std::str;

pub struct Shader {
    pub(super) inner: glow::Program,
    ctx: Rc<glow::Context>,
}

impl Shader {
    pub fn new(ctx: &Context, vert_source: &[u8], frag_source: &[u8]) -> Self {
        let inner = unsafe {
            let program = ctx.inner.create_program().unwrap();

            let vert = ctx.inner.create_shader(glow::VERTEX_SHADER).unwrap();
            let frag = ctx.inner.create_shader(glow::FRAGMENT_SHADER).unwrap();

            ctx.inner.shader_source(vert, str::from_utf8(vert_source).unwrap());
            ctx.inner.shader_source(frag, str::from_utf8(frag_source).unwrap());

            ctx.inner.compile_shader(vert);
            ctx.inner.compile_shader(frag);

            ctx.inner.attach_shader(program, vert);
            ctx.inner.attach_shader(program, frag);

            ctx.inner.link_program(program);

            ctx.inner.delete_shader(vert);
            ctx.inner.delete_shader(frag);

            program
        };

        Self {
            inner,
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
