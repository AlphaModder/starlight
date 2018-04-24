use gfx_hal::Backend;

use graphics::frame::graph::{self, BufferRef, ImageRef};
use graphics::frame::pipeline::*;

use util::MutableLazy;

use std::ops::{Deref, DerefMut, Index};

pub struct ResourcesWrapper<'c, B: Backend> {
    pub template: &'c graph::Resources,
    pub context: &'c RenderContext<'c, B>,
    pub resources: MutableLazy<Resources<B>>,
}

impl<'c, B: Backend> ResourcesWrapper<'c, B> {
    pub fn new(template: &'c graph::Resources, context: &'c RenderContext<'c, B>) -> ResourcesWrapper<'c, B> {
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

impl<'c, B: Backend> Deref for ResourcesWrapper<'c, B> {
    type Target = Resources<B>;
    fn deref(&self) -> &Resources<B> {
        self.resources.get(move || Resources::allocate(self.template))
    }
}

impl<'c, B: Backend> DerefMut for ResourcesWrapper<'c, B> {
    fn deref_mut(&mut self) -> &mut Resources<B> {
        self.resources.get_mut(move || Resources::allocate(self.template))
    }
}

pub struct BufferResource<B: Backend> {
    pub raw: B::Buffer,
}

pub struct ImageResource<B: Backend> {
    pub raw: B::Image,
}

pub struct Resources<B: Backend> {
    buffers: Vec<BufferResource<B>>,
    images: Vec<ImageResource<B>>,
}

impl<B: Backend> Resources<B> {
    pub fn allocate(template: &graph::Resources) -> Resources<B> {
        unimplemented!()
    }

    pub fn get_buffer(&self, reference: &BufferRef) -> &BufferResource<B> {
        &self.buffers[reference.0]
    }

    pub fn get_image(&self, reference: &ImageRef) -> &ImageResource<B> {
        &self.images[reference.0]
    }
}

impl<'a, B: Backend> Index<&'a BufferRef> for Resources<B> {
    type Output = BufferResource<B>;

    fn index(&self, index: &BufferRef) -> &BufferResource<B> {
        self.get_buffer(index)
    }
}

impl<'a, B: Backend> Index<&'a ImageRef> for Resources<B> {
    type Output = ImageResource<B>;

    fn index(&self, index: &ImageRef) -> &ImageResource<B> {
        self.get_image(index)
    }
}
