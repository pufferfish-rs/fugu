/// A rendering pass action.
#[derive(Clone, Copy, Debug)]
pub enum PassAction {
    /// Does nothing.
    Nothing,
    /// Clears the framebuffer.
    Clear {
        /// The color to clear the framebuffer to.
        color: Option<(f32, f32, f32, f32)>,
        /// The depth to clear the framebuffer to.
        depth: Option<f32>,
        /// The stencil to clear the framebuffer to.
        stencil: Option<i32>,
    },
}
