use gfx_hal::Backend;
use graphics::frame::pass::*;
use graphics::frame::{BufferInfo, ImageInfo, BufferRef, ImageRef, ImageLayout};
use graphics::frame::graph::{FrameGraph, Resources};

pub trait BuildGraphicsPass<B: Backend> {
    type Output;
    type Pass: GraphicsPass<B>;
    fn build(&self, &mut GraphicsPassBuilder) -> (Self::Output, Self::Pass);
}

impl<T, B: Backend, O, P: GraphicsPass<B>> BuildGraphicsPass<B> for T 
    where T: Fn(&mut GraphicsPassBuilder) -> (O, P) 
{
    type Output = O;
    type Pass = P;
    fn build(&self, builder: &mut GraphicsPassBuilder) -> (O, P) {
        self(builder)
    }
}

pub struct GraphicsPassBuilder<'r> {
    resources: &'r mut Resources,
}

impl<'r> GraphicsPassBuilder<'r> {
    pub fn new<'p: 'r, B: Backend>(graph: &'r mut FrameGraph<'p, B>) -> Self {
        GraphicsPassBuilder { resources: &mut graph.resources }
    }

    pub fn create_image(&mut self, image: ImageInfo) -> ImageRef {
        unimplemented!()
    }
}

pub trait BuildComputePass<B: Backend> {
    type Output;
    type Pass: ComputePass<B>;
    fn build(&self, &mut ComputePassBuilder) -> (Self::Output, Self::Pass);
}

impl<T, B: Backend, O, P: ComputePass<B>> BuildComputePass<B> for T 
    where T: Fn(&mut ComputePassBuilder) -> (O, P) 
{
    type Output = O;
    type Pass = P;
    fn build(&self, builder: &mut ComputePassBuilder) -> (O, P) {
        self(builder)
    }
}

pub struct ComputePassBuilder<'r> {
    resources: &'r mut Resources,
}

impl<'r> ComputePassBuilder<'r> {
    pub fn new<'p: 'r, B: Backend>(graph: &'r mut FrameGraph<'p, B>) -> Self {
        ComputePassBuilder { resources: &mut graph.resources }
    }
}