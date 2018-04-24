use gfx_hal::Backend;
use gfx_hal::{command, image, queue};
use std::ops::Deref;
use graphics::frame::graph::{ImageRef, BufferRef};
use graphics::frame::pipeline::*;

pub trait GraphicsContext<B: Backend> { // TODO: Any way to eliminate the necessity of this trait would be good
    fn clear_color_image(&mut self, 
        image: &MutableImageResource,
        range: image::SubresourceRange,
        value: command::ClearColor,
    );
}

type GraphicsCommandBuffer<'c, B> = command::CommandBuffer<'c, B, queue::Graphics>;

pub(crate) struct GraphicsContextImpl<'c, 'b: 'c, B: Backend> {
    resources: &'c mut Resources<B>,
    command_buffer: &'c mut GraphicsCommandBuffer<'b, B>,
}

impl<'c, 'b: 'c, B: Backend> GraphicsContextImpl<'c, 'b, B> {
    pub fn new(resources: &'c mut Resources<B>, command_buffer: &'c mut GraphicsCommandBuffer<'b, B>) -> Self {
        GraphicsContextImpl {
            resources: resources,
            command_buffer: command_buffer,
        }
    }
}

impl<'c, 'b, B: Backend> GraphicsContext<B> for GraphicsContextImpl<'c, 'b, B> {
    fn clear_color_image(&mut self, 
        image: &MutableImageResource,
        range: image::SubresourceRange,
        value: command::ClearColor,
    ) {
        self.command_buffer.clear_color_image(&self.resources[&image.id].raw, image.layout, range, value);
    }
}

pub trait ComputeContext<B: Backend> {

}

type ComputeCommandBuffer<'c, B> = command::CommandBuffer<'c, B, queue::Compute>;

pub(crate) struct ComputeContextImpl<'c, 'b: 'c, B: Backend> {
    resources: &'c mut Resources<B>,
    command_buffer: &'c mut ComputeCommandBuffer<'b, B>,
}

impl<'c, 'b: 'c, B: Backend> ComputeContextImpl<'c, 'b, B> {
    pub fn new(resources: &'c mut Resources<B>, command_buffer: &'c mut ComputeCommandBuffer<'b, B>) -> Self {
        ComputeContextImpl {
            resources: resources,
            command_buffer: command_buffer,
        }
    }
}

impl<'c, 'b, B: Backend> ComputeContext<B> for ComputeContextImpl<'c, 'b, B> {
    
}

pub struct ImageResource {
    id: ImageRef,
    layout: image::ImageLayout,
}

pub struct MutableImageResource {
    inner: ImageResource,
}

impl Deref for MutableImageResource {
    type Target = ImageResource;
    fn deref(&self) -> &ImageResource {
        &self.inner
    }
}