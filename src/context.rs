use crate::{
    Buffer, BufferKind, BufferLayout, BufferUsage, PassAction, Pipeline, PipelineInternal, Shader,
    VertexAttribute, Uniform, UniformFormat,
};

use glow::{Framebuffer, HasContext};
use std::cell::RefCell;
use std::mem;
use std::rc::Rc;
use std::slice;

pub(crate) struct ContextState {
    pub pipelines: Vec<PipelineInternal>,
    pub curr_pipeline: Option<usize>,
}

pub struct Context {
    pub(crate) inner: Rc<glow::Context>,
    pub(crate) state: Rc<RefCell<ContextState>>,
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

    pub fn create_buffer(&self, kind: BufferKind, usage: BufferUsage, size: usize) -> Buffer {
        Buffer::new(self, kind, usage, size)
    }

    pub fn create_buffer_with_data<T>(
        &self,
        kind: BufferKind,
        usage: BufferUsage,
        data: &[T],
    ) -> Buffer {
        Buffer::with_data(self, kind, usage, data)
    }

    pub fn create_pipeline(
        &self,
        shader: Shader,
        buffers: &[BufferLayout],
        attrs: &[VertexAttribute],
    ) -> Pipeline {
        Pipeline::new(self, shader, buffers, attrs)
    }

    pub fn create_shader(&self, vert_source: impl AsRef<[u8]>, frag_source: impl AsRef<[u8]>, uniforms: &[Uniform]) -> Shader {
        Shader::new(self, vert_source, frag_source, uniforms)
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

    pub fn set_uniforms<T>(&self, data: T) {
        let pipeline = &self.state.borrow().pipelines[self.state.borrow().curr_pipeline.unwrap()];
        let shader = &pipeline.shader;

        unsafe {
            self.inner.use_program(Some(shader.inner));
        }

        let mut ptr = &data as *const T as *const std::ffi::c_void;
        for uniform in &shader.uniforms {
            match uniform.format {
                UniformFormat::Float1 => {
                    unsafe {
                        self.inner.uniform_1_f32_slice(Some(&uniform.location), slice::from_raw_parts(ptr.cast(), 1));
                        ptr = ptr.offset(mem::size_of::<f32>() as isize);
                    }
                }
                UniformFormat::Float2 => {
                    unsafe {
                        self.inner.uniform_2_f32_slice(Some(&uniform.location), slice::from_raw_parts(ptr.cast(), 2));
                        ptr = ptr.offset(mem::size_of::<[f32; 2]>() as isize);
                    }
                }
                UniformFormat::Float3 => {
                    unsafe {
                        self.inner.uniform_3_f32_slice(Some(&uniform.location), slice::from_raw_parts(ptr.cast(), 3));
                        ptr = ptr.offset(mem::size_of::<[f32; 3]>() as isize);
                    }
                }
                UniformFormat::Float4 => {
                    unsafe {
                        self.inner.uniform_4_f32_slice(Some(&uniform.location), slice::from_raw_parts(ptr.cast(), 4));
                        ptr = ptr.offset(mem::size_of::<[f32; 4]>() as isize);
                    }
                }
                UniformFormat::Int1 => {
                    unsafe {
                        self.inner.uniform_1_i32_slice(Some(&uniform.location), slice::from_raw_parts(ptr.cast(), 1));
                        ptr = ptr.offset(mem::size_of::<i32>() as isize);
                    }
                }
                UniformFormat::Int2 => {
                    unsafe {
                        self.inner.uniform_2_i32_slice(Some(&uniform.location), slice::from_raw_parts(ptr.cast(), 2));
                        ptr = ptr.offset(mem::size_of::<[i32; 2]>() as isize);
                    }
                }
                UniformFormat::Int3 => {
                    unsafe {
                        self.inner.uniform_3_i32_slice(Some(&uniform.location), slice::from_raw_parts(ptr.cast(), 3));
                        ptr = ptr.offset(mem::size_of::<[i32; 3]>() as isize);
                    }
                }
                UniformFormat::Int4 => {
                    unsafe {
                        self.inner.uniform_4_i32_slice(Some(&uniform.location), slice::from_raw_parts(ptr.cast(), 4));
                        ptr = ptr.offset(mem::size_of::<[i32; 4]>() as isize);
                    }
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
