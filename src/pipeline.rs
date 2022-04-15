use super::{Context, Shader};
use alloc::vec;
use alloc::vec::Vec;
use glow::HasContext;

#[derive(Clone, Copy, Debug)]
pub enum VertexStep {
    PerVertex,
    PerInstance(u32),
}

impl Default for VertexStep {
    fn default() -> Self {
        VertexStep::PerVertex
    }
}

#[derive(Clone, Copy, Debug)]
pub enum VertexFormat {
    Float1,
    Float2,
    Float3,
    Float4,
    Byte1,
    Byte2,
    Byte3,
    Byte4,
    Short1,
    Short2,
    Short3,
    Short4,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BufferLayout {
    pub stride: i32,
    pub step_func: VertexStep,
}

#[derive(Clone, Copy, Debug)]
pub struct VertexAttribute {
    pub name: &'static str,
    pub format: VertexFormat,
    pub buffer_index: usize,
}

#[derive(Clone)]
pub(crate) struct VertexAttributeInternal {
    pub location: u32,
    pub format: u32,
    pub offset: i32,
    pub stride: i32,
    pub divisor: u32,
    pub size: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct Pipeline {
    pub(crate) id: usize,
}

pub struct PipelineInternal {
    pub(crate) attrs: Vec<Vec<VertexAttributeInternal>>,
    pub(crate) shader: Shader,
}

impl Pipeline {
    pub(crate) fn new(
        ctx: &Context,
        shader: Shader,
        buffers: &[BufferLayout],
        attrs: &[VertexAttribute],
    ) -> Self {
        let mut attrs_internal = vec![Vec::new(); buffers.len()];
        let mut offsets = vec![0; buffers.len()];
        let mut strides = buffers.iter().map(|e| e.stride).collect::<Vec<_>>();

        for attr in attrs {
            let buffer_index = attr.buffer_index;
            if buffers[buffer_index].stride == 0 {
                strides[buffer_index] += match attr.format {
                    VertexFormat::Float1 => 4,
                    VertexFormat::Float2 => 8,
                    VertexFormat::Float3 => 12,
                    VertexFormat::Float4 => 16,
                    VertexFormat::Byte1 => 1,
                    VertexFormat::Byte2 => 2,
                    VertexFormat::Byte3 => 3,
                    VertexFormat::Byte4 => 4,
                    VertexFormat::Short1 => 2,
                    VertexFormat::Short2 => 4,
                    VertexFormat::Short3 => 6,
                    VertexFormat::Short4 => 8,
                };
            }
        }

        for attr in attrs {
            let buffer_index = attr.buffer_index;

            let size = match attr.format {
                VertexFormat::Float1 => 1,
                VertexFormat::Float2 => 2,
                VertexFormat::Float3 => 3,
                VertexFormat::Float4 => 4,
                VertexFormat::Byte1 => 1,
                VertexFormat::Byte2 => 2,
                VertexFormat::Byte3 => 3,
                VertexFormat::Byte4 => 4,
                VertexFormat::Short1 => 1,
                VertexFormat::Short2 => 2,
                VertexFormat::Short3 => 3,
                VertexFormat::Short4 => 4,
            };

            let offset = offsets[buffer_index];
            offsets[buffer_index] += match attr.format {
                VertexFormat::Float1 => 4,
                VertexFormat::Float2 => 8,
                VertexFormat::Float3 => 12,
                VertexFormat::Float4 => 16,
                VertexFormat::Byte1 => 1,
                VertexFormat::Byte2 => 2,
                VertexFormat::Byte3 => 3,
                VertexFormat::Byte4 => 4,
                VertexFormat::Short1 => 2,
                VertexFormat::Short2 => 4,
                VertexFormat::Short3 => 6,
                VertexFormat::Short4 => 8,
            };
            let stride = strides[buffer_index];

            let divisor = match buffers[buffer_index].step_func {
                VertexStep::PerVertex => 0,
                VertexStep::PerInstance(divisor) => divisor,
            };

            let location = unsafe {
                ctx.inner
                    .get_attrib_location(shader.inner, attr.name)
                    .unwrap()
            };

            let format = match attr.format {
                VertexFormat::Float1 => glow::FLOAT,
                VertexFormat::Float2 => glow::FLOAT,
                VertexFormat::Float3 => glow::FLOAT,
                VertexFormat::Float4 => glow::FLOAT,
                VertexFormat::Byte1 => glow::BYTE,
                VertexFormat::Byte2 => glow::BYTE,
                VertexFormat::Byte3 => glow::BYTE,
                VertexFormat::Byte4 => glow::BYTE,
                VertexFormat::Short1 => glow::SHORT,
                VertexFormat::Short2 => glow::SHORT,
                VertexFormat::Short3 => glow::SHORT,
                VertexFormat::Short4 => glow::SHORT,
            };

            attrs_internal[buffer_index].push(VertexAttributeInternal {
                location,
                format,
                offset,
                stride,
                divisor,
                size,
            });
        }

        let pipelines = &mut ctx.state.borrow_mut().pipelines;
        pipelines.push(PipelineInternal {
            attrs: attrs_internal,
            shader,
        });
        let id = pipelines.len() - 1;

        Pipeline { id }
    }
}
