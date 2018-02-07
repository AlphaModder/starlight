use gfx_hal::Backend;

use graphics::frame::graph;
use graphics::frame::pass::*;
use graphics::frame::{GraphicsContext, ComputeContext};

mod resources;

pub use self::resources::Resources;
use self::resources::ResourcesWrapper;

pub struct RenderContext<'r, B: Backend> {

}

enum RenderPassInstance<'a, B: Backend> {
    Top,
    Graphics(Box<AnyPass<GraphicsContext<B>> + 'a>),
    Compute(Box<AnyPass<ComputeContext<B>> + 'a>),
    Bottom,
    Invalid,
}

impl<'a, B: Backend> RenderPassInstance<'a, B> {
    fn new(node: &'a graph::RenderPassNode) -> RenderPassInstance<'a, B> {
        match *node {
            graph::RenderPassNode::Top => RenderPassInstance::Top,
            graph::RenderPassNode::Graphics(ref pass) => pass.mirror(),
            graph::RenderPassNode::Compute(ref pass) => pass.mirror(),
            graph::RenderPassNode::Bottom => RenderPassInstance::Bottom,
        }
    }
}

pub struct Renderer<'r, B: Backend> {
    context: &'r RenderContext<'r, B>,
    passes: Vec<RenderPassInstance<'r, B>>,
    resources: ResourcesWrapper<'r, B>,
}

impl<'r, B: Backend> Renderer<'r, B> {
    pub fn new<'g: 'r, 'c: 'r>(graph: &'r graph::FrameGraph<'g, B>, context: &'r RenderContext<'c, B>) -> Self {
        Renderer {
            context: context,
            passes: Self::reorder_passes(graph.passes.iter().map(|p| RenderPassInstance::new(p))),
            resources: ResourcesWrapper::new(graph.resources, context),
        }
    }

    fn reorder_passes<I>(passes: I) -> Vec<RenderPassInstance<'r, B>>
        where I: Iterator<Item=RenderPassInstance<'r, B>>
    {
        unimplemented!()
    }
}


