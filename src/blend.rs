/// A blending operation.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendOp {
    Add,
    Subtract,
    ReverseSubtract,
}

/// A blending factor.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendFactor {
    Zero,
    One,
    SourceColor,
    OneMinusSourceColor,
    SourceAlpha,
    OneMinusSourceAlpha,
    DestColor,
    OneMinusDestColor,
    DestAlpha,
    OneMinusDestAlpha,
}

/// A blend equation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlendState {
    /// The blending operation.
    pub op: BlendOp,
    /// The source blending factor.
    pub source: BlendFactor,
    /// The destination blending factor.
    pub dest: BlendFactor,
}

pub(crate) fn gl_blend_op(op: BlendOp) -> u32 {
    match op {
        BlendOp::Add => glow::FUNC_ADD,
        BlendOp::Subtract => glow::FUNC_SUBTRACT,
        BlendOp::ReverseSubtract => glow::FUNC_REVERSE_SUBTRACT,
    }
}

pub(crate) fn gl_blend_factor(factor: BlendFactor) -> u32 {
    match factor {
        BlendFactor::Zero => glow::ZERO,
        BlendFactor::One => glow::ONE,
        BlendFactor::SourceColor => glow::SRC_COLOR,
        BlendFactor::OneMinusSourceColor => glow::ONE_MINUS_SRC_COLOR,
        BlendFactor::SourceAlpha => glow::SRC_ALPHA,
        BlendFactor::OneMinusSourceAlpha => glow::ONE_MINUS_SRC_ALPHA,
        BlendFactor::DestColor => glow::DST_COLOR,
        BlendFactor::OneMinusDestColor => glow::ONE_MINUS_DST_COLOR,
        BlendFactor::DestAlpha => glow::DST_ALPHA,
        BlendFactor::OneMinusDestAlpha => glow::ONE_MINUS_DST_ALPHA,
    }
}
