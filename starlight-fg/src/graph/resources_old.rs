use std::ops::{Index, IndexMut};
use gfx_hal::{buffer, image, format};


use crate::graph::{PassRef, AttachmentInfo};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BufferRef(pub(crate) usize);

pub struct BufferResource {
    pub usage: buffer::Usage,
    pub writer: PassRef,
    pub readers: Vec<PassRef>,
    pub(crate) _priv: (),
}

bitflags! {
    pub struct ImageAccessFlags: u16 {
        const TRANSFER = 0x1;
        const ATTACHMENT = 0x2;
    }
}

pub enum ImageRead {
    Transfer,
    Attachment(AttachmentInfo),
}

pub enum ImageWrite {
    Transfer,
    Attachment(AttachmentInfo, bool),
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ImageRef(pub(crate) usize);

pub struct ImageCreateInfo {
    kind: image::Kind,
    mip_levels: image::Level,
    format: format::Format,
    tiling: image::Tiling,
    usage: image::Usage,
}

pub struct ImageResource {
    pub info: ImageCreateInfo,
    pub writer: (PassRef, ImageWrite),
    pub readers: Vec<(PassRef, ImageRead)>,
    pub(crate) _priv: (),
}

#[derive(Default)]
pub struct Resources {
    pub(crate) buffers: Vec<BufferResource>,
    pub(crate) images: Vec<ImageResource>,
}

impl Resources {
    pub(crate) fn push_buffer(&mut self, buffer: BufferResource) -> BufferRef {
        self.buffers.push(buffer);
        BufferRef(self.buffers.len() - 1)
    }

    pub(crate) fn push_image(&mut self, image: ImageResource) -> ImageRef {
        self.images.push(image);
        ImageRef(self.images.len() - 1)
    }
}

impl Index<BufferRef> for Resources {
    type Output = BufferResource;
    fn index(&self, buffer: BufferRef) -> &BufferResource {
        &self.buffers[buffer.0]
    }
}

impl IndexMut<BufferRef> for Resources {
    fn index_mut(&mut self, buffer: BufferRef) -> &mut BufferResource {
        &mut self.buffers[buffer.0]
    }
}

impl Index<ImageRef> for Resources {
    type Output = ImageResource;
    fn index(&self, image: ImageRef) -> &ImageResource {
        &self.images[image.0]
    }
}

impl IndexMut<ImageRef> for Resources {
    fn index_mut(&mut self, image: ImageRef) -> &mut ImageResource {
        &mut self.images[image.0]
    }
}