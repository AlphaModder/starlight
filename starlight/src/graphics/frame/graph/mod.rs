use gfx_hal::Backend;
use graphics::frame::pass::*;
use graphics::frame::{GraphicsContext, ComputeContext};

mod resources;
mod build;

pub use self::resources::*;
use self::build::*;

pub enum RenderPassNode<'f, B> {
    Top,
    Graphics(Box<AnyPassOwned<GraphicsContext<B>> + 'f>),
    Compute(Box<AnyPassOwned<ComputeContext<B>> + 'f>),
    Bottom,
    Invalid,
}

pub struct FrameGraph<'f, B: Backend> {
    passes: Vec<RenderPassNode<'f, B>>,
    resources: Resources,
}

impl<'f, B: Backend> FrameGraph<'f, B> {
    fn add_graphics_pass(&mut self) -> GraphicsPassBuilder<B> {
        GraphicsPassBuilder { 
            graph: self
        }
    }

    fn add_compute_pass(&mut self) -> ComputePassBuilder<B> {
        ComputePassBuilder { 
            graph: self
        }
    }
}

