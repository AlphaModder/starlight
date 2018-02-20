use gfx_hal::Backend;

use graphics::frame::graph;

mod resources;

use self::resources::*;

pub struct RenderContext<'r, B: Backend> {
    device: &'r B::Device,
}

pub struct Renderer<'r, B: Backend> {
    context: &'r RenderContext<'r, B>,
    passes: Vec<&'r graph::RenderPass<'r, B>>,
    resources: ResourcesWrapper<'r, B>,
}

impl<'r, B: Backend> Renderer<'r, B> {
    pub fn new<'g: 'r, 'c: 'r>(graph: &'r graph::FrameGraph<'g, B>, context: &'r RenderContext<'c, B>) -> Self {
        Renderer {
            context: context,
            passes: Self::reorder_passes(graph.passes.iter()),
            resources: ResourcesWrapper::new(&graph.resources, context),
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
}


