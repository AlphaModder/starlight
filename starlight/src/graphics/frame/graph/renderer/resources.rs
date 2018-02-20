use gfx_hal::Backend;

use util::MutableLazy;
use graphics::frame::graph;

use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

pub struct ResourcesWrapper<'r, B: Backend> {
    pub template: &'r graph::Resources,
    pub context: &'r graph::RenderContext<'r, B>,
    pub resources: MutableLazy<Resources<B>>,
}

impl<'r, B: Backend> ResourcesWrapper<'r, B> {
    pub fn new(template: &'r graph::Resources, context: &'r graph::RenderContext<'r, B>) -> ResourcesWrapper<'r, B> {
        ResourcesWrapper { 
            template: template, 
            context: context, 
            resources: MutableLazy::new()
        }
    }

    pub fn allocate(&self) {
        Deref::deref(self);
    }
}

impl<'r, B: Backend> Deref for ResourcesWrapper<'r, B> {
    type Target = Resources<B>;
    fn deref(&self) -> &Resources<B> {
        let resources = Resources::allocate(self.template);
        self.resources.get(move || resources)
    }
}

impl<'r, B: Backend> DerefMut for ResourcesWrapper<'r, B> {
    fn deref_mut(&mut self) -> &mut Resources<B> {
        let resources = Resources::allocate(self.template);
        self.resources.get_mut(move || resources)
    }
}

pub struct Resources<B: Backend> {
    phantom: PhantomData<B>
}

unsafe impl<B: Backend> Sync for Resources<B> { }

impl<B: Backend> Resources<B> {
    fn allocate(template: &graph::Resources) -> Resources<B> {
        unimplemented!()
    }
}

