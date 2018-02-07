use graphics::frame::{BufferInfo, ImageInfo};
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct PassRef(pub ::petgraph::graph::NodeIndex);

pub struct BufferResourceDef {
    info: BufferInfo,
    creator: PassRef,
    name: String,
}

pub struct ImageResourceDef {
    info: ImageInfo,
    creator: PassRef,
    name: String,
}

pub struct BufferRef(usize);
impl BufferRef {
    fn clone(&self) -> BufferRef { BufferRef(self.0) }
}

pub struct ImageRef(usize);
impl ImageRef {
    fn clone(&self) -> ImageRef { ImageRef(self.0) }
}

#[derive(Default)]
pub struct Resources {
    buffers: Vec<BufferResourceDef>,
    images: Vec<ImageResourceDef>,
}

impl Resources { 
    fn add_buffer(&mut self, buffer: BufferResourceDef) -> BufferRef {
        self.buffers.push(buffer);
        BufferRef(self.buffers.len() - 1)
    }

    fn get_buffer(&self, reference: &BufferRef) -> &BufferResourceDef {
        &self.buffers[reference.0]
    }

    fn get_buffer_mut(&mut self, reference: &BufferRef) -> &mut BufferResourceDef {
        &mut self.buffers[reference.0]
    }

    fn add_image(&mut self, image: ImageResourceDef) -> ImageRef {
        self.images.push(image);
        ImageRef(self.images.len() - 1)
    }

    fn get_image(&self, reference: &ImageRef) -> &ImageResourceDef {
        &self.images[reference.0]
    }

    fn get_image_mut(&mut self, reference: &ImageRef) -> &mut ImageResourceDef {
        &mut self.images[reference.0]
    }
}

impl<'a> Index<&'a BufferRef> for Resources {
    type Output = BufferResourceDef;

    fn index(&self, index: &BufferRef) -> &BufferResourceDef {
        self.get_buffer(index)
    }
}

impl<'a> Index<&'a ImageRef> for Resources {
    type Output = ImageResourceDef;

    fn index(&self, index: &ImageRef) -> &ImageResourceDef {
        self.get_image(index)
    }
}

impl<'a> IndexMut<&'a BufferRef> for Resources {
    fn index_mut(&mut self, index: &BufferRef) -> &mut BufferResourceDef {
        self.get_buffer_mut(index)
    }
}

impl<'a> IndexMut<&'a ImageRef> for Resources {
    fn index_mut(&mut self, index: &ImageRef) -> &mut ImageResourceDef {
        self.get_image_mut(index)
    }
}