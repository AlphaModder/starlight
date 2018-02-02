use starlight::graphics::backend;
use starlight::graphics::frame::{self, FrameGraph};

type Backend = backend::Vulkan;

fn main() {
    let frame_graph: FrameGraph<Backend> = FrameGraph::new();
    
}

pub struct GBufferOutputs {
    emissive: ImageRef,
    albedo: ImageRef,
    normal: ImageRef,
    pbr: ImageRef,
    depth: ImageRef,
}

fn add_gbuffer_pass(graph: &mut FrameGraph) -> GBufferOutputs {
    graph.add_graphics_pass(|builder| {
        
    })
}