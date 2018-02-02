use super::graph::FrameGraph;
use typemap;

use gfx_hal::Backend;

use std::ops::{Deref, DerefMut};
use std::cell::RefCell;

use self::traits::*;
mod traits {
    use gfx_hal::queue::QueueType;
    use gfx_hal::queue::QueueType::*;
    pub trait QueueTypeExt {
        fn is_subtype(&self, q: &QueueType) -> bool;
    }

    impl QueueTypeExt for QueueType {
        fn is_subtype(&self, q: &QueueType) -> bool {
            match (*self, *q) {
                (General, General) => false,
                (General, _) => true,
                (Graphics, Transfer) | (Compute, Transfer) => true,
                _ => false,
            }
        }
    }
}

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

pub struct FrameRenderer<'r, B: Backend> {
    graph: &'r FrameGraph<'r, B>,
    context: &'r RenderContext<'r, B>,
    resources: FrameResourcesWrapper<'r, B>,
}

impl<'r, B: Backend> FrameRenderer<'r, B> {

    pub fn new<'g: 'r, 'c: 'r>(graph: &'r FrameGraph<'g, B>, context: &'r RenderContext<'c, B>) -> Self {
        FrameRenderer {
            graph: graph,
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