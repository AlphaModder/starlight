extern crate starlight;

use starlight::graphics::backend;
use starlight::graphics::frame::{buffer, image, format};
use starlight::graphics::frame::*;

type Backend = <backend::Vulkan as backend::Backend>::GfxBackend;

fn main() {
    let mut graph: FrameGraph<Backend> = FrameGraph::new();
    let gbuffer_pass = graph.add_graphics_pass(|builder: &mut GraphicsPassBuilder| {
        let outputs = GBufferOutputs {
            emissive: unimplemented!(),
            albedo: unimplemented!(),
            normal: unimplemented!(),
            pbr: unimplemented!(),
            depth: unimplemented!(),
        };
        let executor = |context: &mut GraphicsContext<Backend>| {
            
        };
        (outputs, executor)
    });
    let lighting_pass = graph.add_graphics_pass(|builder: &mut GraphicsPassBuilder| {
        let hdr = builder.write_image(gbuffer_pass.emissive);
        let albedo = builder.read_image(&gbuffer_pass.albedo);
        let normal = builder.read_image(&gbuffer_pass.normal);
        let pbr = builder.read_image(&gbuffer_pass.pbr);
        let depth = builder.read_image(&gbuffer_pass.depth);
        let executor = |context: &mut GraphicsContext<Backend>| {

        };
        (hdr, executor)
    });
}

pub struct GBufferOutputs {
    emissive: ImageRef,
    albedo: ImageRef,
    normal: ImageRef,
    pbr: ImageRef,
    depth: ImageRef,
}