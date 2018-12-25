use crate::{BufferRef, ImageRef};

use std::collections::HashMap;
use std::borrow::Borrow;

use gfx_hal::{buffer, image, Backend};
use gfx_hal::command::{
    CommandBuffer, MultiShot, Primary, ClearColor, ClearDepthStencil, DescriptorSetOffset
};
use gfx_hal::queue::{Graphics, Compute};


struct ImageState<B: Backend> {
    handle: B::Image,
    layout: image::Layout,
}

struct BufferState<B: Backend> {
    handle: B::Buffer,
}

struct ResourceHandles<B: Backend> {
    images: HashMap<ImageRef, ImageState<B>>,
    buffers: HashMap<BufferRef, BufferState<B>>,
}

impl<B: Backend> ResourceHandles<B> {
    fn get_image_state(&self, image: ImageRef) -> &ImageState<B> {
        self.images.get(&image).expect("Invalid image handle!")
    }

    fn get_buffer_state(&self, buffer: BufferRef) -> &BufferState<B> {
        self.buffers.get(&buffer).expect("Invalid buffer handle!")
    }
}

pub struct GraphicsContext<'c, B: Backend> {
    buffer: CommandBuffer<'c, B, Graphics, MultiShot, Primary>,
    resources: ResourceHandles<B>,
}

impl<'c, B: Backend> GraphicsContext<'c, B> {
    pub fn clear_image<T>(
        &mut self,
        image: ImageRef,
        color: ClearColor,
        depth_stencil: ClearDepthStencil,
        subresource_ranges: T,
    ) where
        T: IntoIterator,
        T::Item: Borrow<image::SubresourceRange>,
    {
        let image = self.resources.get_image_state(image);
        self.buffer.clear_image(&image.handle, image.layout, color, depth_stencil, subresource_ranges)
    }

    /*
    pub fn bind_index_buffer(&mut self, ibv: buffer::IndexBufferView<B>) {
        self.buffer.bind_index_buffer(ibv)
    }
    */

    pub fn bind_vertex_buffers<I, T>(&mut self, first_binding: u32, buffers: I)
    where
        I: IntoIterator<Item = (BufferRef, buffer::Offset)>,
    {
        self.buffer.bind_vertex_buffers(first_binding, buffers.into_iter().map(
            |(b, o)| (&self.resources.get_buffer_state(b).handle, o)
        ));
    }

    pub fn bind_pipeline(&mut self, pipeline: &B::GraphicsPipeline) {
        self.buffer.bind_graphics_pipeline(pipeline)
    }

    pub fn bind_descriptor_sets<I, J>(
        &mut self,
        layout: &B::PipelineLayout,
        first_set: usize,
        sets: I,
        offsets: J,
    ) where
        I: IntoIterator,
        I::Item: Borrow<B::DescriptorSet>,
        J: IntoIterator,
        J::Item: Borrow<DescriptorSetOffset>,
    {
        self.buffer.bind_graphics_descriptor_sets(layout, first_set, sets, offsets)
    }


}