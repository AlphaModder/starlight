mod resource;
mod resource2;
mod pass;

use gfx_hal::{Backend};
use std::collections::HashMap;
use petgraph::Graph;

pub use self::resource::*;
pub use self::pass::*;

struct FrameResources {
    buffers: Vec<BufferResource>,
    images: Vec<ImageResource>,
}

pub struct Frame<'f, B: Backend> {
    render_passes: Vec<Box<RenderPass<B> + 'f>>,
    resources: FrameResources,
}

impl<'f, B: Backend> Frame<'f, B> {

    pub fn add_pass<'d, D: RenderPassDef<'d, B>>(&'d mut self, def: D) -> D::Output {
        let mut builder = PassBuilder::new(self.render_passes.len(), &mut self.resources);
        let (output, pass) = def.setup_pass(&mut builder);
        self.render_passes.push(pass);
        output
    }

    fn render<'d>(&self, context: &RenderContext<'d, B>) {
        let pass_order = petgraph::algo::toposort(self.construct_graph(), None).unwrap();
    }

    fn order_passes(&self) {
        
    }

    fn construct_graph<'d>(&self) -> Graph<usize, ()> {
        let mut graph = Graph::new();
        for i in 0..self.render_passes.len() { graph.add_node(i); };
        for buffer in self.resources.buffers {
            if let Some(ResourceCreator::Pass(p)) = buffer.creator {
                for reader in buffer.readers {
                    graph.add_edge(p, reader, ());
                }
            }
        }
        for image in self.resources.images {
            if let Some(ResourceCreator::Pass(p)) = image.creator {
                for reader in image.readers {
                    graph.add_edge(p, reader, ());
                }
            }
        }
        graph
    }
    
}

/*
pub struct PassBuilder<'r> {
    resources: &'r mut FrameResources,
    buffer_inputs: Vec<BufferRef>,
    image_inputs: Vec<(ImageRef, ImageLayout)>,
    buffer_outputs: Vec<BufferRef>,
    image_outputs: Vec<ImageRef>,
}

impl<'r> PassBuilder<'r> {

    fn new(resources: &'r mut FrameResources) -> PassBuilder {
        PassBuilder {
            resources: resources,
            buffer_inputs: Vec::new(),
            image_inputs: Vec::new(),
            buffer_outputs: Vec::new(),
            image_outputs: Vec::new(),
        }
    }

    pub fn create_buffer(&mut self, name: &str, info: BufferInfo) -> BufferRef {
        let reference = BufferRef(self.resources.buffers.len());
        self.resources.buffers.push(BufferResource::new(name, info));
        self.buffer_outputs.push(reference.clone());
        reference
    }

    pub fn read_buffer(&mut self, buffer: &BufferRef) {
        self.buffer_inputs.push(buffer.clone());
    }

    pub fn write_buffer(&mut self, buffer: BufferRef, new_name: &str) -> BufferRef {
        self.read_buffer(&buffer);
        let info = self.resources.buffers[buffer.0].info;
        self.create_buffer(new_name, info)
    }

    pub fn create_image(&mut self, name: &str, info: ImageInfo) -> ImageRef {
        let reference = ImageRef(self.resources.images.len());
        self.resources.images.push(ImageResource::new(name, info));
        self.image_outputs.push(reference.clone());
        reference
    }

    pub fn read_image(&mut self, image: &ImageRef, layout: ImageLayout) {
        self.image_inputs.push((image.clone(), layout));
    }

    pub fn write_image(&mut self, image: ImageRef, layout: ImageLayout, new_name: &str) -> ImageRef {
        self.read_image(&image, layout);
        let info = self.resources.images[image.0].info;
        self.create_image(new_name, info)
    }

    fn into_pass<'f, B: Backend>(self, pass: Box<RenderPass<B> + 'f>) -> RenderPassData<'f, B> {
        RenderPassData {
            pass: pass,
            buffer_inputs: self.buffer_inputs,
            image_inputs: self.image_inputs,
            buffer_outputs: self.buffer_outputs,
            image_outputs: self.image_outputs,
        }
    }
}
*/

pub struct PassBuilder<'r> {
    pass: usize,
    resources: &'r mut FrameResources,
}

impl<'r> PassBuilder<'r> {

    fn new(pass: usize, resources: &'r mut FrameResources) -> PassBuilder<'r> {
        PassBuilder {
            pass: pass,
            resources: resources,
        }
    }

    pub fn create_buffer(&mut self, name: &str, info: BufferInfo) -> BufferRef {
        self.resources.buffers.push(BufferResource {
            info: info,
            readers: Vec::new(),
            data: ResourceData {
                name: name.to_string(),
                creator: Some(ResourceCreator::Pass(self.pass)),
                consumer: None,
            }
        });
        BufferRef(self.resources.buffers.len() - 1)
    }

    pub fn read_buffer(&mut self, buffer: &BufferRef) {
        self.resources.buffers[buffer.0].readers.push(self.pass);
    }

    pub fn write_buffer(&mut self, mut buffer: BufferRef, new_name: &str) -> BufferRef {
        self.read_buffer(&buffer);
        self.resources.buffers[buffer.0].data.consumer = Some(ResourceConsumer::Pass(self.pass));
        let info = self.resources.buffers[buffer.0].info;
        self.create_buffer(new_name, info)
    }

    pub fn create_image(&mut self, name: &str, info: ImageInfo) -> ImageRef {
        self.resources.images.push(ImageResource {
            info: info,
            readers: HashMap::new(),
            data: ResourceData {
                name: name.to_string(),
                creator: Some(ResourceCreator::Pass(self.pass)),
                consumer: None,
            }
        });
        ImageRef(self.resources.images.len() - 1)
    }

    pub fn read_image(&mut self, image: &ImageRef, layout: ImageLayout) {
        self.resources.images[image.0].readers.insert(self.pass, layout);
    }

    pub fn write_image(&mut self, mut image: ImageRef, layout: ImageLayout, new_name: &str) -> ImageRef {
        self.read_image(&image, layout);
        self.resources.images[image.0].data.consumer = Some(ResourceConsumer::Pass(self.pass));
        let info = self.resources.images[image.0].info;
        self.create_image(new_name, info)
    }
}

pub struct RenderContext<'d, B: Backend> {
    device: &'d B::Device,
}

impl<'d, B: Backend> RenderContext<'d, B> {
    
}