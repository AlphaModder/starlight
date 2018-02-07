use gfx_hal::Backend;

use graphics::frame::FrameGraph;

pub struct GraphicsPassBuilder<'f, B: Backend> {
    graph: &'f mut FrameGraph<'f, B>
}

impl<'f, B: Backend> GraphicsPassBuilder<'f, B> {
    fn build(self) {

    }
}

pub struct ComputePassBuilder<'f, B: Backend> {
    graph: &'f mut FrameGraph<'f, B>
}

impl<'f, B: Backend> ComputePassBuilder<'f, B> {
    fn build(self) {
        
    }
}