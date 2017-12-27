use gfx_hal::{buffer, image, format};
use std;
use std::collections::HashMap;
use std::iter::Iterator;

/*
mod frame_resource {
    use super::*;
    use self::FrameResource::*;
    pub(crate) enum FrameResource {
        Buffer(BufferResource),
        Image(ImageResource),
    }

    impl FrameResource {
        pub fn data(&self) -> &ResourceData {
            match *self {
                Buffer(ref b) => &b.data,
                Image(ref i) => &i.data,
            }
        }

        pub fn readers<'r>(&'r self) -> ReadersIter<'r> { 
            match *self {
                Buffer(ref b) => ReadersIter::Buffer(b.readers.iter()),
                Image(ref i) => ReadersIter::Image(i.readers.keys()),
            }
        }
    }

    pub(crate) enum ReadersIter<'r> {
        Buffer(std::slice::Iter<'r, usize>),
        Image(std::collections::hash_map::Keys<'r, usize, ImageLayout>)
    }

    impl<'r> Iterator for ReadersIter<'r> {
        type Item = &'r usize;

        fn next(&mut self) -> Option<&'r usize> {
            match *self {
                ReadersIter::Buffer(ref mut i) => i.next(),
                ReadersIter::Image(ref mut i) => i.next(),
            }
        }
    }
}
*/

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum ResourceCreator {
    Input,
    Pass(usize),
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum ResourceConsumer {
    Pass(usize),
    Output,
}

pub(crate) struct ResourceData {
    pub name: String,
    pub creator: Option<ResourceCreator>,
    pub consumer: Option<ResourceConsumer>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct BufferInfo {
    size: u64,
    stride: u64,
    usage: buffer::Usage,
}

pub struct BufferResource {
    pub(crate) info: BufferInfo,
    pub(crate) readers: Vec<usize>,
    pub(crate) data: ResourceData,
}

pub struct BufferRef(pub(crate) usize);
impl BufferRef {
    pub(crate) fn clone(&self) -> BufferRef { BufferRef(self.0) }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ImageInfo {
    kind: image::Kind,
    level: image::Level,
    format: format::Format,
    usage: image::Usage,
}

pub struct ImageResource {
    pub(crate) info: ImageInfo,
    pub(crate) readers: HashMap<usize, ImageLayout>,
    pub(crate) data: ResourceData,
}

pub struct ImageRef(pub(crate) usize);
impl ImageRef {
    pub(crate) fn clone(&self) -> ImageRef { ImageRef(self.0) }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum ImageLayout {
    Color,
    Resolve,
    DepthStencil,
    Texture,
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