use graphics::resource;
use graphics::command::CommandList;

struct FrameGraph {

}

impl FrameGraph {
    fn add_render_pass<T>(
        setup: fn() -> T,
        execute: fn(T) -> CommandList
    ) {
        unimplemented!()
    }
}

struct FrameGraphBuilder {
    
}

impl FrameGraphBuilder {
    fn read_buffer(buffer: &FrameGraphBufferResource) -> resource::Buffer {
        unimplemented!()
    }
}

struct FrameGraphBufferResource {

}