use graphics::frame::graph;

use gfx_hal::{Backend, Device};
use gfx_hal::{queue, command, pool};

mod resources;
// mod context;

use self::resources::*;
use self::context::*;

pub struct RenderContext<'r, B: Backend> {
    device: &'r B::Device,
}

pub struct Renderer<'r, B: Backend> {
    context: &'r RenderContext<'r, B>,
    passes: Vec<&'r graph::RenderPass<'r, B>>,
    resources: ResourcesWrapper<'r, B>,
    command_recorder: ParallelCommandRecorder<'r, B>
}

impl<'r, B: Backend> Renderer<'r, B> {
    pub fn new<'g: 'r, 'c: 'r>(graph: &'r graph::FrameGraph<'g, B>, context: &'r RenderContext<'c, B>) -> Self {
        Renderer {
            context: context,
            passes: Self::reorder_passes(graph.passes.iter()),
            resources: ResourcesWrapper::new(&graph.resources, context),
            command_recorder: ParallelCommandRecorder::new(context.device, pool::CommandPoolCreateFlags::empty())
        }
    }

    pub fn allocate_resources(&mut self) {
        self.resources.allocate();
    }

    /// TODO: Proper deallocation of resources.
    pub fn deallocate_resources(&mut self) {
        self.resources = ResourcesWrapper::new(self.resources.template, self.resources.context);
    }

    fn reorder_passes<I>(passes: I) -> Vec<&'r graph::RenderPass<'r, B>>
        where I: Iterator<Item=&'r graph::RenderPass<'r, B>>
    {
        unimplemented!()
    }

    pub fn render(&mut self) {
        
    }

    fn record_command_buffers(&mut self) {
         
    }
}






