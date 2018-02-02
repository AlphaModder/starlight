use super::*;

use petgraph::Graph;
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct PassRef(pub ::petgraph::graph::NodeIndex);

pub struct BufferResource {
    info: BufferInfo,
    creator: PassRef,
    name: String,
}

pub struct ImageResource {
    info: ImageInfo,
    creator: PassRef,
    name: String,
}

pub struct BufferRef(usize);
impl BufferRef {
    fn clone(&self) -> BufferRef { BufferRef(self.0) }
}

pub struct ImageRef(usize);
impl ImageRef {
    fn clone(&self) -> ImageRef { ImageRef(self.0) }
}

#[derive(Default)]
pub struct Resources {
    buffers: Vec<BufferResource>,
    images: Vec<ImageResource>,
}

impl Resources { 
    fn add_buffer(&mut self, buffer: BufferResource) -> BufferRef {
        self.buffers.push(buffer);
        BufferRef(self.buffers.len() - 1)
    }

    fn get_buffer(&self, reference: &BufferRef) -> &BufferResource {
        &self.buffers[reference.0]
    }

    fn get_buffer_mut(&mut self, reference: &BufferRef) -> &mut BufferResource {
        &mut self.buffers[reference.0]
    }

    fn add_image(&mut self, image: ImageResource) -> ImageRef {
        self.images.push(image);
        ImageRef(self.images.len() - 1)
    }

    fn get_image(&self, reference: &ImageRef) -> &ImageResource {
        &self.images[reference.0]
    }

    fn get_image_mut(&mut self, reference: &ImageRef) -> &mut ImageResource {
        &mut self.images[reference.0]
    }
}

impl<'a> Index<&'a BufferRef> for Resources {
    type Output = BufferResource;

    fn index(&self, index: &BufferRef) -> &BufferResource {
        self.get_buffer(index)
    }
}

impl<'a> Index<&'a ImageRef> for Resources {
    type Output = ImageResource;

    fn index(&self, index: &ImageRef) -> &ImageResource {
        self.get_image(index)
    }
}

impl<'a> IndexMut<&'a BufferRef> for Resources {
    fn index_mut(&mut self, index: &BufferRef) -> &mut BufferResource {
        self.get_buffer_mut(index)
    }
}

impl<'a> IndexMut<&'a ImageRef> for Resources {
    fn index_mut(&mut self, index: &ImageRef) -> &mut ImageResource {
        self.get_image_mut(index)
    }
}

pub enum RenderPass<'f, B> {
    Top,
    Graphics(Box<GraphicsPass<B> + 'f>),
    Compute(Box<ComputePass<B> + 'f>),
    Bottom,
    Invalid,
}

pub enum Dependency {
    Buffer(BufferRef),
    Image(ImageRef, ImageLayout)
}

pub struct FrameGraph<'f, B: Backend> {
    pub(super) resources: Resources,
    pub(super) graph: Graph<RenderPass<'f, B>, Dependency>,
    pub(super) top: PassRef,
    pub(super) bottom: PassRef,
}

impl<'f, B: Backend> FrameGraph<'f, B> {

    pub fn new() -> FrameGraph<'f, B> {
        let mut graph = Graph::default();
        let top = PassRef(graph.add_node(RenderPass::Top));
        let bottom = PassRef(graph.add_node(RenderPass::Bottom));
        FrameGraph {
            resources: Default::default(),
            graph: graph,
            top: top,
            bottom: bottom,
        }
    }

    pub fn add_graphics_pass<D: GraphicsPassDef<B>>(&mut self, def: &D) -> D::Output {
        let pass_ref = PassRef(self.graph.add_node(RenderPass::Invalid));
        let (output, pass) = {
            let mut builder = PassBuilder::new(pass_ref, self);
            def.setup_pass(&mut builder)
        };
        self.graph[pass_ref.0] = RenderPass::Graphics(pass);
        output
    }

    pub fn add_compute_pass<D: ComputePassDef<B>>(&mut self, def: &D) -> D::Output {
        let pass_ref = PassRef(self.graph.add_node(RenderPass::Invalid));
        let (output, pass) = {
            let mut builder = PassBuilder::new(pass_ref, self);
            def.setup_pass(&mut builder)
        };
        self.graph[pass_ref.0] = RenderPass::Compute(pass);
        output
    }

    pub fn add_input_buffer(&mut self, name: &str, info: BufferInfo) -> BufferRef {
        self.resources.add_buffer(BufferResource {
            info: info,
            creator: self.top,
            name: name.to_string(),
        })
    }

    pub fn add_input_image(&mut self, name: &str, info: ImageInfo) -> ImageRef {
        self.resources.add_image(ImageResource {
            info: info,
            creator: self.top,
            name: name.to_string(),
        })
    }

    pub fn make_buffer_output(&mut self, buffer: BufferRef) {
        self.graph.add_edge(
            self.resources[&buffer].creator.0,
            self.bottom.0,
            Dependency::Buffer(buffer.clone())
        );
    }

    pub fn make_image_output(&mut self, image: ImageRef, layout: ImageLayout) {
        self.graph.add_edge(
            self.resources[&image].creator.0,
            self.bottom.0,
            Dependency::Image(image.clone(), layout)
        );
    }

}

pub struct PassBuilder<'r, 'f: 'r, B: Backend> {
    pass: PassRef,
    frame: &'r mut FrameGraph<'f, B>,
}

impl<'r, 'f, B: Backend> PassBuilder<'r, 'f, B> {

    fn new(pass: PassRef, frame: &'r mut FrameGraph<'f, B>) -> Self {
        PassBuilder {
            pass: pass,
            frame: frame,
        }
    }

    pub fn create_buffer(&mut self, name: &str, info: BufferInfo) -> BufferRef {
        self.frame.resources.add_buffer(BufferResource {
            info: info,
            creator: self.pass,
            name: name.to_string(),
        })
    }

    pub fn read_buffer(&mut self, buffer: &BufferRef) {
        self.frame.graph.add_edge(
            self.frame.resources[buffer].creator.0, 
            self.pass.0,
            Dependency::Buffer(buffer.clone())
        );
    }

    pub fn write_buffer(&mut self, buffer: BufferRef, new_name: &str) -> BufferRef {
        self.read_buffer(&buffer);
        let info = self.frame.resources.buffers[buffer.0].info;
        self.create_buffer(new_name, info)
    }

    pub fn create_image(&mut self, name: &str, info: ImageInfo) -> ImageRef {
        self.frame.resources.add_image(ImageResource {
            info: info,
            creator: self.pass,
            name: name.to_string()
        })
    }

    pub fn read_image(&mut self, image: &ImageRef, layout: ImageLayout) {
        self.frame.graph.add_edge(
            self.pass.0,
            self.frame.resources[image].creator.0, 
            Dependency::Image(image.clone(), layout)
        );
    }

    pub fn write_image(&mut self, image: ImageRef, new_name: &str, layout: ImageLayout) -> ImageRef {
        self.read_image(&image, layout);
        let info = self.frame.resources.images[image.0].info;
        self.create_image(new_name, info)
    }
}