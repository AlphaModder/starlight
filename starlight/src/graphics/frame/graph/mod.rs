use gfx_hal::Backend;
use graphics::frame::pass::*;

use std::ops::Deref;

mod resources;
mod build;

pub use self::resources::*;
pub use self::build::*;

pub(crate) enum RenderPass<'p, B> {
    Top,
    Graphics(Box<GraphicsPass<B> + 'p>),
    Compute(Box<ComputePass<B> + 'p>),
    Bottom,
    Invalid,
}

pub struct FrameGraph<'p, B: Backend> {
    pub(crate) passes: Vec<RenderPass<'p, B>>,
    pub(crate) resources: Resources,
}

impl<'p, B: Backend> Default for FrameGraph<'p, B> {
    fn default() -> FrameGraph<'p, B> {
        FrameGraph {
            passes: Default::default(),
            resources: Default::default(),
        }
    }
}

impl<'p, B: Backend> FrameGraph<'p, B> {

    pub fn new() -> FrameGraph<'p, B> {
        Default::default()
    }

    pub fn add_graphics_pass<T: BuildGraphicsPass<B>>(&mut self, build: T) -> Guard<T::Output>
        where T::Pass: 'p, 
    {
        let (output, pass) = build.build(&mut GraphicsPassBuilder::new(self));
        self.passes.push(RenderPass::Graphics(Box::new(pass)));
        Guard(output)
    }

    pub fn add_compute_pass<T: BuildComputePass<B>>(&mut self, build: T) -> Guard<T::Output>
        where T::Pass: 'p,
    {
        let (output, pass) = build.build(&mut ComputePassBuilder::new(self));
        self.passes.push(RenderPass::Compute(Box::new(pass)));
        Guard(output)
    }

}

pub struct Guard<T>(T);

impl<T> Deref for Guard<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}