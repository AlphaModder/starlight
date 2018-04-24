mod pass;
mod graph;
mod pipeline;

pub use self::pass::*;
pub use self::graph::*;

pub use gfx_hal::{buffer, image, format};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct BufferInfo {
    /*
    pub size: u64,
    pub stride: u64,
    pub usage: buffer::Usage,
    */
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ImageInfo {
    /*
    pub kind: image::Kind,
    pub level: image::Level,
    pub format: format::Format,
    pub usage: image::Usage,
    */
}

impl Default for ImageInfo {
    fn default() -> ImageInfo { unimplemented!() }
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
    Attachment,
    Texture,
}
