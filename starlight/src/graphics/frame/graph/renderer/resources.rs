use gfx_hal::Backend;

use util::MutableLazy;
use graphics::frame::graph::{self, BufferRef, ImageRef};

use std::ops::{Deref, DerefMut, Index};

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
        self.resources.get(move || Resources::allocate(self.template))
    }
}

impl<'r, B: Backend> DerefMut for ResourcesWrapper<'r, B> {
    fn deref_mut(&mut self) -> &mut Resources<B> {
        self.resources.get_mut(move || Resources::allocate(self.template))
    }
}

struct BufferResource<B: Backend> {
    pub raw: B::Buffer,
}

struct ImageResource<B: Backend> {
    pub raw: B::Image,
}

pub struct Resources<B: Backend> {
    buffers: Vec<BufferResource<B>>,
    images: Vec<ImageResource<B>>,
}

impl<B: Backend> Resources<B> {
    fn allocate(template: &graph::Resources) -> Resources<B> {
        unimplemented!()
    }

    fn get_buffer(&self, reference: &BufferRef) -> &BufferResource<B> {
        &self.buffers[reference.0]
    }

    fn get_image(&self, reference: &ImageRef) -> &ImageResource<B> {
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
