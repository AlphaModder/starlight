use gfx_hal::{buffer, image, format};

use crate::graph::{PassRef, AttachmentInfo};

#[derive(Eq, PartialEq)]
pub struct BufferRef(pub(crate) PassRef, pub(crate) usize);

pub struct BufferResource {
    pub usage: buffer::Usage,
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

#[derive(Eq, PartialEq)]
pub struct ImageRef(pub(crate) PassRef, pub(crate) usize);

pub struct ImageCreateInfo {
    kind: image::Kind,
    mip_levels: image::Level,
    format: format::Format,
    tiling: image::Tiling,
    usage: image::Usage,
}

pub struct ImageResource {
    pub info: ImageCreateInfo,
    pub write_type: ImageWrite,
}