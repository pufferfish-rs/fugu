use super::Context;
use glow::HasContext;
use std::{mem, rc::Rc, slice};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferKind {
    Vertex,
    Index,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferUsage {
    Static,
    Dynamic,
    Stream,
}

pub struct Buffer {
    pub(super) inner: glow::Buffer,
    pub(super) kind: BufferKind,
    size: usize,
    ctx: Rc<glow::Context>,
}

impl Buffer {
    pub fn new(ctx: &Context, kind: BufferKind, usage: BufferUsage, size: usize) -> Self {
        assert_ne!(
            usage,
            BufferUsage::Static,
            "Static buffers must be initialized with data"
        );

        let target = match kind {
            BufferKind::Vertex => glow::ARRAY_BUFFER,
            BufferKind::Index => glow::ELEMENT_ARRAY_BUFFER,
        };
        let usage = match usage {
            BufferUsage::Static => glow::STATIC_DRAW,
            BufferUsage::Dynamic => glow::DYNAMIC_DRAW,
            BufferUsage::Stream => glow::STREAM_DRAW,
        };

        let inner = unsafe {
            let buffer = ctx.inner.create_buffer().unwrap();
            ctx.inner.bind_buffer(target, Some(buffer));
            ctx.inner.buffer_data_size(target, size as _, usage);
            buffer
        };

        Self {
            inner,
            kind,
            size,
            ctx: ctx.inner.clone(),
        }
    }

    pub fn with_data<T>(ctx: &Context, kind: BufferKind, usage: BufferUsage, data: &[T]) -> Self {
        let target = match kind {
            BufferKind::Vertex => glow::ARRAY_BUFFER,
            BufferKind::Index => glow::ELEMENT_ARRAY_BUFFER,
        };
        let usage = match usage {
            BufferUsage::Static => glow::STATIC_DRAW,
            BufferUsage::Dynamic => glow::DYNAMIC_DRAW,
            BufferUsage::Stream => glow::STREAM_DRAW,
        };

        let size = mem::size_of_val(data);

        let inner = unsafe {
            let buffer = ctx.inner.create_buffer().unwrap();
            ctx.inner.bind_buffer(target, Some(buffer));
            let data = slice::from_raw_parts(data.as_ptr() as *const u8, size);
            ctx.inner.buffer_data_u8_slice(target, data, usage);
            buffer
        };

        Self {
            inner,
            kind,
            size,
            ctx: ctx.inner.clone(),
        }
    }

    pub fn update<T>(&self, data: &[T]) {
        let size = mem::size_of_val(data);
        assert!(
            size <= self.size,
            "Update data cannot be larger than the buffer"
        );

        let target = match self.kind {
            BufferKind::Vertex => glow::ARRAY_BUFFER,
            BufferKind::Index => glow::ELEMENT_ARRAY_BUFFER,
        };

        unsafe {
            self.ctx.bind_buffer(target, Some(self.inner));
            let data = slice::from_raw_parts(data.as_ptr() as *const u8, size);
            self.ctx.buffer_sub_data_u8_slice(target, 0, data);
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            self.ctx.delete_buffer(self.inner);
        }
    }
}
