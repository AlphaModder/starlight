use super::graph::{FrameGraph, AnyPass, PassRef, RenderPassNode};
use super::context::*;

use typemap;

use gfx_hal::Backend;

use std::ops::{Deref, DerefMut};
use std::cell::RefCell;
use std::collections::HashMap;

pub struct RenderContext<'d, B: Backend> {
    device: &'d B::Device,
    queues: typemap::TypeMap,
}

struct FrameResourcesWrapper<'r, B: Backend> {
    graph: &'r FrameGraph<'r, B>,
    context: &'r RenderContext<'r, B>,
    resources: RefCell<Option<FrameResources<B>>>,
}

impl<'r, B: Backend> FrameResourcesWrapper<'r, B> {
    fn new(graph: &'r FrameGraph<'r, B>, context: &'r RenderContext<'r, B>) -> FrameResourcesWrapper<'r, B> {
        FrameResourcesWrapper { graph: graph, context: context, resources: RefCell::new(None) }
    }

    fn allocate_resources(&self) {
        if self.resources.borrow().is_some() { return; }

    }
}

impl<'r, B: Backend> Deref for FrameResourcesWrapper<'r, B> {
    type Target = FrameResources<B>;
    fn deref(&self) -> &FrameResources<B> {
        self.allocate_resources();
        &self.resources.borrow().unwrap()
    }
}

impl<'r, B: Backend> DerefMut for FrameResourcesWrapper<'r, B> {
    fn deref_mut(&mut self) -> &mut FrameResources<B> {
        self.allocate_resources();
        &mut self.resources.borrow_mut().unwrap()
    }
}

pub struct FrameResources<B: Backend> {

}

enum PassStorage<'r, B> {
    Graphics(Box<AnyPass<GraphicsContext<B>> + 'r>),
    Compute(Box<AnyPass<ComputeContext<B>> + 'r>),
}

pub struct FrameRenderer<'r, B: Backend> {
    context: &'r RenderContext<'r, B>,
    graph: &'r FrameGraph<'r, B>,
    passes: HashMap<PassRef, PassStorage<'r, B>>,
    resources: FrameResourcesWrapper<'r, B>,
}

impl<'r, B: Backend> FrameRenderer<'r, B> {
    pub fn new<'g: 'r, 'c: 'r>(graph: &'r FrameGraph<'g, B>, context: &'r RenderContext<'c, B>) -> Self {
        let passes = HashMap::from_iter(graph.node_references().filter_map(|(i, n)| {
            match n {
                RenderPassNode::Graphics(ref pass) => Some((PassRef(i), PassStorage::Graphics(pass.new_ref()))),
                RenderPassNode::Compute(ref pass) => Some((PassRef(i), PassStorage::Compute(pass.new_ref()))),
                _ => None,
            }
        }));

        FrameRenderer {
            graph: graph,
            passes: passes,
            context: context,
            resources: FrameResourcesWrapper::new(graph, context),
        }
    }

    pub fn allocate_resources(&mut self) {
        self.resources.allocate_resources();
    }

    pub fn render_frame(&mut self) {
        
    }
}