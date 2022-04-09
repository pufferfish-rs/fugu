use super::{Buffer, PassAction, Pipeline, PipelineInternal};
use glow::{Framebuffer, HasContext};
use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

pub(super) struct ContextState {
    pub pipelines: Vec<PipelineInternal>,
    pub curr_pipeline: Option<usize>,
}

pub struct Context {
    pub(super) inner: Rc<glow::Context>,
    pub(super) state: Rc<RefCell<ContextState>>,
    default_framebuffer: Framebuffer,
    // TODO: should we cache GL state?
}

impl Context {
    pub fn new<F>(loader_function: F) -> Self
    where
        F: FnMut(&str) -> *const std::ffi::c_void,
    {
        let inner = Rc::new(unsafe { glow::Context::from_loader_function(loader_function) });
        unsafe {
            let vao = inner.create_vertex_array().unwrap();
            inner.bind_vertex_array(Some(vao));
        }

        // TODO: fix this (blocked by grovesNL/glow#187)
        let default_framebuffer =
            unsafe { mem::transmute(inner.get_parameter_i32(glow::FRAMEBUFFER_BINDING) as u32) };

        let state = Rc::new(RefCell::new(ContextState {
            pipelines: Vec::new(),
            curr_pipeline: None,
        }));

        Self {
            inner,
            default_framebuffer,
            state,
        }
    }

    pub fn set_pipeline(&self, pipeline: &Pipeline) {
        self.state.borrow_mut().curr_pipeline = Some(pipeline.id);
        unsafe {
            self.inner.use_program(Some(
                self.state.borrow().pipelines[pipeline.id].shader.inner,
            ));
        }
        // TODO: other stuff
    }

    pub fn set_vertex_buffer(&self, buffer: &Buffer) {
        self.set_vertex_buffers(&[buffer]);
    }

    pub fn set_vertex_buffers(&self, buffers: &[&Buffer]) {
        let pipeline = &self.state.borrow().pipelines[self.state.borrow().curr_pipeline.unwrap()];
        for (buffer_index, attrs) in pipeline.attrs.iter().enumerate() {
            unsafe {
                self.inner
                    .bind_buffer(glow::ARRAY_BUFFER, Some(buffers[buffer_index].inner));
                for attr in attrs {
                    self.inner.enable_vertex_attrib_array(attr.location);
                    self.inner.vertex_attrib_pointer_f32(
                        attr.location,
                        attr.size,
                        attr.format,
                        false,
                        attr.stride,
                        attr.offset,
                    );
                    self.inner
                        .vertex_attrib_divisor(attr.location, attr.divisor);
                }
            }
        }
    }

    pub fn draw(&self, start: usize, vertices: usize, instances: usize) {
        unsafe {
            self.inner.draw_arrays_instanced(
                glow::TRIANGLES,
                start as _,
                vertices as _,
                instances as _,
            );
        }
    }

    pub fn begin_default_pass(&self, action: PassAction) {
        match action {
            PassAction::Nothing => {}
            PassAction::Clear {
                color,
                depth,
                stencil,
            } => unsafe {
                let mut clear_flag = 0;
                if let Some((r, g, b, a)) = color {
                    self.inner.clear_color(r, g, b, a);
                    clear_flag |= glow::COLOR_BUFFER_BIT;
                }
                if let Some(depth) = depth {
                    self.inner.clear_depth_f32(depth);
                    clear_flag |= glow::DEPTH_BUFFER_BIT;
                }
                if let Some(stencil) = stencil {
                    self.inner.clear_stencil(stencil);
                    clear_flag |= glow::STENCIL_BUFFER_BIT;
                }
                self.inner.clear(clear_flag);
            },
        }
    }

    pub fn end_render_pass(&self) {
        unsafe {
            self.inner
                .bind_framebuffer(glow::FRAMEBUFFER, Some(self.default_framebuffer));
        }
    }

    pub fn commit_frame(&self) {
        unsafe {
            self.inner.bind_buffer(glow::ARRAY_BUFFER, None);
            self.inner.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
            // TODO: clear texture bindings?
        }
    }

    pub fn set_viewport(&self, x: u32, y: u32, width: u32, height: u32) {
        unsafe {
            self.inner.viewport(x as _, y as _, width as _, height as _);
        }
    }
}
