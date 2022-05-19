use alloc::rc::Rc;

use glow::HasContext;

use crate::Context;

pub struct Image {
    pub(crate) inner: glow::Texture,
    ctx: Rc<glow::Context>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Rgb8,
    Rgba8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFilter {
    Nearest,
    Linear,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageWrap {
    Clamp,
    Repeat,
}

impl Image {
    pub(crate) fn new(
        ctx: &Context,
        width: u32,
        height: u32,
        pixel_format: ImageFormat,
        filter: ImageFilter,
        wrap: ImageWrap,
    ) -> Self {
        let inner = unsafe {
            let texture = ctx.inner.create_texture().unwrap();
            ctx.inner.bind_texture(glow::TEXTURE_2D, Some(texture)); // TODO: other texture types

            let format = match pixel_format {
                ImageFormat::Rgb8 => glow::RGB,
                ImageFormat::Rgba8 => glow::RGBA,
            };
            let kind = match pixel_format {
                ImageFormat::Rgb8 | ImageFormat::Rgba8 => glow::UNSIGNED_BYTE,
            };
            let filter = match filter {
                ImageFilter::Nearest => glow::NEAREST,
                ImageFilter::Linear => glow::LINEAR,
            };
            let wrap = match wrap {
                ImageWrap::Clamp => glow::CLAMP_TO_EDGE,
                ImageWrap::Repeat => glow::REPEAT,
            };

            ctx.inner.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                format as _,
                width as _,
                height as _,
                0,
                format,
                kind,
                None,
            );
            ctx.inner.generate_mipmap(glow::TEXTURE_2D);

            ctx.inner
                .tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, filter as _);
            ctx.inner
                .tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, filter as _);
            ctx.inner
                .tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, wrap as _);
            ctx.inner
                .tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, wrap as _);

            texture
        };

        Self {
            inner,
            ctx: ctx.inner.clone(),
        }
    }

    pub(crate) fn with_data(
        ctx: &Context,
        width: u32,
        height: u32,
        pixel_format: ImageFormat,
        filter: ImageFilter,
        wrap: ImageWrap,
        data: &[u8],
    ) -> Self {
        let inner = unsafe {
            let texture = ctx.inner.create_texture().unwrap();
            ctx.inner.bind_texture(glow::TEXTURE_2D, Some(texture)); // TODO: other texture types

            let format = match pixel_format {
                ImageFormat::Rgb8 => glow::RGB,
                ImageFormat::Rgba8 => glow::RGBA,
            };
            let kind = match pixel_format {
                ImageFormat::Rgb8 | ImageFormat::Rgba8 => glow::UNSIGNED_BYTE,
            };
            let filter = match filter {
                ImageFilter::Nearest => glow::NEAREST,
                ImageFilter::Linear => glow::LINEAR,
            };
            let wrap = match wrap {
                ImageWrap::Clamp => glow::CLAMP_TO_EDGE,
                ImageWrap::Repeat => glow::REPEAT,
            };

            ctx.inner.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                format as _,
                width as _,
                height as _,
                0,
                format,
                kind,
                Some(data),
            );
            ctx.inner.generate_mipmap(glow::TEXTURE_2D);

            ctx.inner
                .tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, filter as _);
            ctx.inner
                .tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, filter as _);
            ctx.inner
                .tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, wrap as _);
            ctx.inner
                .tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, wrap as _);

            texture
        };

        Self {
            inner,
            ctx: ctx.inner.clone(),
        }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            self.ctx.delete_texture(self.inner);
        }
    }
}
