use typemap;
use gfx_hal::{buffer, image, format};
use gfx_hal::Backend;

mod graph;
mod pass;

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

use self::traits::*;
pub use self::pass::*;
pub use self::graph::*;


/*
pub struct PassBuilder<'r> {
    resources: &'r mut FrameResources,
    buffer_inputs: Vec<BufferRef>,
    image_inputs: Vec<(ImageRef, ImageLayout)>,
    buffer_outputs: Vec<BufferRef>,
    image_outputs: Vec<ImageRef>,
}

impl<'r> PassBuilder<'r> {

    fn new(resources: &'r mut FrameResources) -> PassBuilder {
        PassBuilder {
            resources: resources,
            buffer_inputs: Vec::new(),
            image_inputs: Vec::new(),
            buffer_outputs: Vec::new(),
            image_outputs: Vec::new(),
        }
    }

    pub fn create_buffer(&mut self, name: &str, info: BufferInfo) -> BufferRef {
        let reference = BufferRef(self.resources.buffers.len());
        self.resources.buffers.push(BufferResourceData::new(name, info));
        self.buffer_outputs.push(reference.clone());
        reference
    }

    pub fn read_buffer(&mut self, buffer: &BufferRef) {
        self.buffer_inputs.push(buffer.clone());
    }

    pub fn write_buffer(&mut self, buffer: BufferRef, new_name: &str) -> BufferRef {
        self.read_buffer(&buffer);
        let info = self.resources.buffers[buffer.0].info;
        self.create_buffer(new_name, info)
    }

    pub fn create_image(&mut self, name: &str, info: ImageInfo) -> ImageRef {
        let reference = ImageRef(self.resources.images.len());
        self.resources.images.push(ImageResourceData::new(name, info));
        self.image_outputs.push(reference.clone());
        reference
    }

    pub fn read_image(&mut self, image: &ImageRef, layout: ImageLayout) {
        self.image_inputs.push((image.clone(), layout));
    }

    pub fn write_image(&mut self, image: ImageRef, layout: ImageLayout, new_name: &str) -> ImageRef {
        self.read_image(&image, layout);
        let info = self.resources.images[image.0].info;
        self.create_image(new_name, info)
    }

    fn into_pass<'f, B: Backend>(self, pass: Box<RenderPass<B> + 'f>) -> RenderPassData<'f, B> {
        RenderPassData {
            pass: pass,
            buffer_inputs: self.buffer_inputs,
            image_inputs: self.image_inputs,
            buffer_outputs: self.buffer_outputs,
            image_outputs: self.image_outputs,
        }
    }
}
*/

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct BufferInfo {
    size: u64,
    stride: u64,
    usage: buffer::Usage,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ImageInfo {
    kind: image::Kind,
    level: image::Level,
    format: format::Format,
    usage: image::Usage,
}

/*
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ImageViewInfo {
    image: ImageInfo,
    format: format::Format,
    swizzle: format::Swizzle,
    // subresource_range: image::SubresourceRange,
}
*/

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum ImageLayout {
    Color,
    Resolve,
    DepthStencil,
    Texture,
}

pub struct RenderContext<'d, B: Backend> {
    device: &'d B::Device,
    queues: typemap::TypeMap,
}

/*
struct Capability<'a, B: Backend, C: queue::Capability + 'a>(PhantomData<&'a (B, C)>);
impl<'a, B: Backend, C: queue::Capability> typemap::Key for Capability<'a, B, C> {
    type Value = Vec<&'a mut queue::CommandQueue<B, C>>;
}

impl<'d, B: Backend> RenderContext<'d, B> {
    pub(crate) fn acquire_queue<C: queue::Capability>(&mut self) -> Option<&mut queue::CommandQueue<B, C>> {
        let queues = match self.queues.get_mut::<Capability<'d, B, C>>() {
            Some(q) => q,
            None => return None,
        };
        unimplemented!()
    }
}
*/

