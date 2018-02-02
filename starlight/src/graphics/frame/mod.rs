use gfx_hal::{buffer, image, format};
use gfx_hal::Backend;

mod context;
mod pass;
mod graph;
mod renderer;

pub use self::context::*;
pub use self::pass::*;
pub use self::graph::{FrameGraph, BufferRef, ImageRef};
pub use self::renderer::*;

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
    Attachment,
    Texture,
}




