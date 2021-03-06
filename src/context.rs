use alloc::rc::Rc;
use alloc::slice;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::{ffi, mem};

use glow::{Framebuffer, HasContext};

use crate::{
    gl_blend_factor, gl_blend_op, BlendState, Buffer, BufferKind, BufferLayout, BufferUsage, Image,
    ImageFilter, ImageFormat, ImageUniform, ImageWrap, PassAction, Pipeline, PipelineInternal,
    Shader, Uniform, UniformFormat, VertexAttribute,
};

pub(crate) struct ContextState {
    pub pipelines: Vec<PipelineInternal>,
    pub curr_pipeline: Option<usize>,
    pub idx_buffer_set: bool,
}

/// A rendering context.
pub struct Context {
    pub(crate) inner: Rc<glow::Context>,
    pub(crate) state: Rc<RefCell<ContextState>>,
    default_framebuffer: Framebuffer,
    // TODO: should we cache GL state?
}

impl Context {
    /// Creates a new context from a OpenGL loader function.
    pub fn new<F>(loader_function: F) -> Self
    where
        F: FnMut(&str) -> *const ffi::c_void,
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
            idx_buffer_set: false,
        }));

        Self {
            inner,
            default_framebuffer,
            state,
        }
    }

    /// Creates and returns a new [`Buffer`].
    pub fn create_buffer(&self, kind: BufferKind, usage: BufferUsage, size: usize) -> Buffer {
        Buffer::new(self, kind, usage, size)
    }

    /// Creates and returns a new [`Buffer`] with the given data.
    pub fn create_buffer_with_data<T>(
        &self,
        kind: BufferKind,
        usage: BufferUsage,
        data: &[T],
    ) -> Buffer {
        Buffer::with_data(self, kind, usage, data)
    }

    /// Creates and returns a new [`Image`].
    pub fn create_image(
        &self,
        width: u32,
        height: u32,
        format: ImageFormat,
        filter: ImageFilter,
        wrap: ImageWrap,
    ) -> Image {
        Image::new(self, width, height, format, filter, wrap)
    }

    /// Creates and returns a new [`Image`] with the given data.
    pub fn create_image_with_data(
        &self,
        width: u32,
        height: u32,
        format: ImageFormat,
        filter: ImageFilter,
        wrap: ImageWrap,
        data: &[u8],
    ) -> Image {
        Image::with_data(self, width, height, format, filter, wrap, data)
    }

    /// Creates and returns a new [`Pipeline`].
    pub fn create_pipeline(
        &self,
        shader: Shader,
        buffers: &[BufferLayout],
        attrs: &[VertexAttribute],
    ) -> Pipeline {
        Pipeline::new(self, shader, buffers, attrs)
    }

    /// Creates and returns a new [`Shader`] with the given shader source.
    ///
    /// Shader translation based on the rendering backend must be handled by the
    /// caller.
    pub fn create_shader(
        &self,
        vert_source: impl AsRef<[u8]>,
        frag_source: impl AsRef<[u8]>,
        uniforms: &[Uniform],
        images: &[ImageUniform],
    ) -> Shader {
        Shader::new(self, vert_source, frag_source, uniforms, images)
    }

    /// Sets the current pipeline.
    pub fn set_pipeline(&self, pipeline: &Pipeline) {
        self.state.borrow_mut().curr_pipeline = Some(pipeline.id);
        unsafe {
            self.inner.use_program(Some(
                self.state.borrow().pipelines[pipeline.id].shader.inner,
            ));
        }
        // TODO: other stuff
    }

    /// Sets the current vertex buffer.
    pub fn set_vertex_buffer(&self, buffer: &Buffer) {
        self.set_vertex_buffers(&[buffer]);
    }

    /// Sets the current vertex buffers.
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

    /// Sets the current index buffer.
    pub fn set_index_buffer(&self, buffer: &Buffer) {
        unsafe {
            self.inner
                .bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(buffer.inner));
        }
        self.state.borrow_mut().idx_buffer_set = true;
    }

    /// Sets shader uniforms.
    pub fn set_uniforms<T>(&self, data: T) {
        let pipeline = &self.state.borrow().pipelines[self.state.borrow().curr_pipeline.unwrap()];
        let shader = &pipeline.shader;

        let mut ptr = &data as *const T as *const ffi::c_void;
        for uniform in &shader.uniforms {
            match uniform.format {
                UniformFormat::Float1 => unsafe {
                    self.inner.uniform_1_f32_slice(
                        Some(&uniform.location),
                        slice::from_raw_parts(ptr.cast(), 1),
                    );
                    ptr = ptr.add(mem::size_of::<f32>());
                },
                UniformFormat::Float2 => unsafe {
                    self.inner.uniform_2_f32_slice(
                        Some(&uniform.location),
                        slice::from_raw_parts(ptr.cast(), 2),
                    );
                    ptr = ptr.add(mem::size_of::<[f32; 2]>());
                },
                UniformFormat::Float3 => unsafe {
                    self.inner.uniform_3_f32_slice(
                        Some(&uniform.location),
                        slice::from_raw_parts(ptr.cast(), 3),
                    );
                    ptr = ptr.add(mem::size_of::<[f32; 3]>());
                },
                UniformFormat::Float4 => unsafe {
                    self.inner.uniform_4_f32_slice(
                        Some(&uniform.location),
                        slice::from_raw_parts(ptr.cast(), 4),
                    );
                    ptr = ptr.add(mem::size_of::<[f32; 4]>());
                },
                UniformFormat::Int1 => unsafe {
                    self.inner.uniform_1_i32_slice(
                        Some(&uniform.location),
                        slice::from_raw_parts(ptr.cast(), 1),
                    );
                    ptr = ptr.add(mem::size_of::<i32>());
                },
                UniformFormat::Int2 => unsafe {
                    self.inner.uniform_2_i32_slice(
                        Some(&uniform.location),
                        slice::from_raw_parts(ptr.cast(), 2),
                    );
                    ptr = ptr.add(mem::size_of::<[i32; 2]>());
                },
                UniformFormat::Int3 => unsafe {
                    self.inner.uniform_3_i32_slice(
                        Some(&uniform.location),
                        slice::from_raw_parts(ptr.cast(), 3),
                    );
                    ptr = ptr.add(mem::size_of::<[i32; 3]>());
                },
                UniformFormat::Int4 => unsafe {
                    self.inner.uniform_4_i32_slice(
                        Some(&uniform.location),
                        slice::from_raw_parts(ptr.cast(), 4),
                    );
                    ptr = ptr.add(mem::size_of::<[i32; 4]>());
                },
            }
        }
    }

    /// Sets shader images.
    pub fn set_images(&self, images: &[&Image]) {
        let pipeline = &self.state.borrow().pipelines[self.state.borrow().curr_pipeline.unwrap()];
        let shader = &pipeline.shader;

        for (i, image_uniform) in shader.image_uniforms.iter().enumerate() {
            unsafe {
                self.inner.active_texture(glow::TEXTURE0 + i as u32);
                self.inner
                    .bind_texture(glow::TEXTURE_2D, Some(images[i].inner));
                self.inner
                    .uniform_1_i32(Some(&image_uniform.location), i as i32);
            }
        }
    }

    /// Draws geometry from the current vertex and index buffers.
    ///
    /// # Arguments
    /// * `start` - The index of the first vertex to draw.
    /// * `count` - The number of vertices to draw.
    /// * `instances` - The number of instances to draw.
    pub fn draw(&self, start: usize, count: usize, instances: usize) {
        unsafe {
            if self.state.borrow().idx_buffer_set {
                self.inner.draw_elements_instanced(
                    glow::TRIANGLES,
                    count as _,
                    glow::UNSIGNED_SHORT,
                    start as i32 * 2,
                    instances as _,
                );
            } else {
                self.inner.draw_arrays_instanced(
                    glow::TRIANGLES,
                    start as _,
                    count as _,
                    instances as _,
                );
            }
        }
    }

    /// Begins the default render pass with the given [`PassAction`].
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

    /// Ends the current render pass.
    pub fn end_render_pass(&self) {
        unsafe {
            self.inner
                .bind_framebuffer(glow::FRAMEBUFFER, Some(self.default_framebuffer));
        }
    }

    /// Commits everything that was drawn and performs cleanup.
    pub fn commit_frame(&self) {
        unsafe {
            self.inner.bind_buffer(glow::ARRAY_BUFFER, None);
            self.inner.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
            // TODO: clear texture bindings?
        }

        self.state.borrow_mut().curr_pipeline = None;
        self.state.borrow_mut().idx_buffer_set = false;
    }

    /// Sets the viewport.
    pub fn set_viewport(&self, x: u32, y: u32, width: u32, height: u32) {
        unsafe {
            self.inner.viewport(x as _, y as _, width as _, height as _);
        }
    }

    /// Sets the current blend state.
    pub fn set_blend(&self, state: BlendState) {
        unsafe {
            self.inner.enable(glow::BLEND);
            self.inner.blend_equation(gl_blend_op(state.op));
            self.inner
                .blend_func(gl_blend_factor(state.source), gl_blend_factor(state.dest));
        }
    }

    /// Sets the current blend state separately for color and alpha.
    pub fn set_blend_separate(&self, color: BlendState, alpha: BlendState) {
        unsafe {
            self.inner.enable(glow::BLEND);
            self.inner
                .blend_equation_separate(gl_blend_op(color.op), gl_blend_op(alpha.op));
            self.inner.blend_func_separate(
                gl_blend_factor(color.source),
                gl_blend_factor(color.dest),
                gl_blend_factor(alpha.source),
                gl_blend_factor(alpha.dest),
            );
        }
    }
}
