use crate::graph::{Resources, BufferRef, ImageRef};

pub fn can_unify_buffers(resources: &graph::Resources, buffer1: BufferRef, buffer2: BufferRef) -> bool {
    
}

pub struct PhysicalResources {
    image_table: HashMap<ImageRef, usize>,
    buffer_table: HashMap<BufferRef, usize>,
    physical_images: Vec<Image>,
    physical_buffers: Vec<Buffer>,
}

impl PhysicalResources {
    fn get_image(&self, image: ImageRef) -> &Image { 
        return self.physical_images[self.image_table[image]];
    }

    fn get_buffer(&self, buffer: BufferRef) -> &Buffer { 
        return self.physical_buffers[self.buffer_table[buffer]];
    }
}

struct Image {
    handle: B::Image,

}

struct Buffer {
    handle B::Buffer,
}