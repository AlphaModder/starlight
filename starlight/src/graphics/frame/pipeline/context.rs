use gfx_hal::Backend;
use gfx_hal::{command, image, queue};

use std::ops::Deref;
use std::borrow::Borrow;

use graphics::frame::pipeline::resources::Resources;
use graphics::frame::{ImageRef, BufferRef};

type GraphicsCommandBuffer<'c, B> = command::CommandBuffer<'c, B, queue::Graphics>;

pub struct GraphicsContext<'c, 'b: 'c, B: Backend> {
    resources: &'c Resources<B>,
    command_buffer: &'c mut GraphicsCommandBuffer<'b, B>,
}

impl<'c, 'b: 'c, B: Backend> GraphicsContext<'c, 'b, B> {
    pub(crate) fn new(resources: &'c Resources<B>, command_buffer: &'c mut GraphicsCommandBuffer<'b, B>) -> Self {
        GraphicsContext {
            resources: resources,
            command_buffer: command_buffer,
        }
    }

    pub fn clear_image<T>(
        &mut self,
        image: &MutableImageResource,
        color: command::ClearColor,
        depth_stencil: command::ClearDepthStencil,
        subresource_ranges: T,
    ) where
        T: IntoIterator,
        T::Item: Borrow<image::SubresourceRange> {
        self.command_buffer.clear_image(&self.resources[&image.id].raw, image.layout, color, depth_stencil, subresource_ranges);
    }
}

type ComputeCommandBuffer<'c, B> = command::CommandBuffer<'c, B, queue::Compute>;

pub struct ComputeContext<'c, 'b: 'c, B: Backend> {
    resources: &'c Resources<B>,
    command_buffer: &'c mut ComputeCommandBuffer<'b, B>,
}

impl<'c, 'b: 'c, B: Backend> ComputeContext<'c, 'b, B> {
    pub(crate) fn new(resources: &'c Resources<B>, command_buffer: &'c mut ComputeCommandBuffer<'b, B>) -> Self {
        ComputeContext {
            resources: resources,
            command_buffer: command_buffer,
        }
    }
}

pub struct ImageResource {
    id: ImageRef,
    layout: image::Layout,
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