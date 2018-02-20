extern crate starlight;

use starlight::graphics::backend;
use starlight::graphics::frame::{buffer, image, format};
use starlight::graphics::frame::*;

type Backend = <backend::Vulkan as backend::Backend>::GfxBackend;

fn main() {
    let mut graph: FrameGraph<Backend> = FrameGraph::new();
    let gbuffer_pass = graph.add_graphics_pass(|builder: &mut GraphicsPassBuilder| {
        let outputs = GBufferOutputs {
            emissive: builder.create_image(ImageInfo {
                
            }),
            albedo: unimplemented!(),
            normal: unimplemented!(),
            pbr: unimplemented!(),
            depth: unimplemented!(),
        };
        let executor = |context: &mut GraphicsContext<Backend>| {

        };
        (outputs, executor)
    });
}

pub struct GBufferOutputs {
    emissive: ImageRef,
    albedo: ImageRef,
    normal: ImageRef,
    pbr: ImageRef,
    depth: ImageRef,
}