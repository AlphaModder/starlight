use gfx_hal::Backend;

use graphics::frame::graph;
use graphics::frame::renderer::*;

use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

pub struct Resources<B: Backend> {
    
}

impl<B: Backend> Resources<B> {
    fn allocate(template: &graph::Resources) -> Resources<B> {
        
    }
}

pub struct ResourcesWrapper<'r, B: Backend> {
    template: &'r graph::Resources,
    context: &'r RenderContext<'r, B>,
    resources: RefCell<Option<Resources<B>>>,
}

impl<'r, B: Backend> ResourcesWrapper<'r, B> {
    fn new(template: &'r graph::Resources, context: &'r RenderContext<'r, B>) -> ResourcesWrapper<'r, B> {
        ResourcesWrapper { template: template, context: context, resources: RefCell::new(None) }
    }

    fn allocate_resources(&self) {
        if self.resources.borrow().is_some() { return; }
        *self.resources.borrow_mut() = Some(Resources::allocate(self.template));
    }
}

impl<'r, B: Backend> Deref for ResourcesWrapper<'r, B> {
    type Target = Resources<B>;
    fn deref(&self) -> &Resources<B> {
        self.allocate_resources();
        &self.resources.borrow().unwrap()
    }
}

impl<'r, B: Backend> DerefMut for ResourcesWrapper<'r, B> {
    fn deref_mut(&mut self) -> &mut Resources<B> {
        self.allocate_resources();
        &mut self.resources.borrow_mut().unwrap()
    }
}
