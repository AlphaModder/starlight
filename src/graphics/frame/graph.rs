use super::{
    BufferInfo, ImageInfo, ImageLayout, RenderPass, RenderPassDef, Backend
};

use petgraph;
use petgraph::Graph;
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone)]
struct PassRef(petgraph::graph::NodeIndex);

struct ResourceLifetime {
    creator: PassRef,
    consumer: Option<PassRef>,
}

struct BufferResource {
    info: BufferInfo,
    lifetime: ResourceLifetime,
    name: String,
}

struct ImageResource {
    info: ImageInfo,
    lifetime: ResourceLifetime,
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
struct Resources {
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

enum RenderStage<'f, B> {
    Top,
    RenderPass(Box<RenderPass<B> + 'f>),
    Bottom,
    Invalid,
}

enum Dependency {
    Buffer(BufferRef),
    Image(ImageRef, ImageLayout)
}

pub struct FrameGraph<'f, B: Backend> {
    resources: Resources,
    graph: Graph<RenderStage<'f, B>, Dependency, petgraph::Directed>,
    top: PassRef,
    bottom: PassRef,
}

impl<'f, B: Backend> FrameGraph<'f, B> {
    /*
    pub(crate) fn new(builder: FrameBuilder<'f, B>) -> FrameGraph<'f, B> {
        let mut resources = Resources { 
            buffers: Vec::new(),
            images: Vec::new(),
        };
        let mut graph = Graph::with_capacity(builder.render_passes.len(), 0);

        let top = graph.add_node(RenderStage::Top);
        let pass_refs = Vec::new();
        for pass in builder.render_passes {
            pass_refs.push(graph.add_node(RenderStage::RenderPass(pass)));
        }
        let bottom = graph.add_node(RenderStage::Bottom);

        for buffer in builder.resources.buffers {
            let new_buffer = resources.add_buffer(BufferResource { info: buffer.info });
            if let Some(ResourceCreator::Pass(p)) = buffer.data.creator {
                for reader in buffer.readers {
                    graph.add_edge(
                        pass_refs[p], 
                        pass_refs[reader], 
                        Dependency::Buffer(new_buffer),
                    );
                }
            }
        }
        for image in builder.resources.images {
            let new_image = resources.add_image(ImageResource { info: image.info });
            if let Some(ResourceCreator::Pass(p)) = image.data.creator {
                for reader in image.readers.into_iter() {
                    graph.add_edge(
                        pass_refs[p], 
                        pass_refs[reader.0], 
                        Dependency::Image(new_image, reader.1),
                    );
                }
            }
        }

        FrameGraph {
            resources: resources,
            graph: graph
        }
    }
    */

    pub fn new() -> FrameGraph<'f, B> {
        let mut graph = Graph::default();
        let top = PassRef(graph.add_node(RenderStage::Top));
        let bottom = PassRef(graph.add_node(RenderStage::Bottom));
        FrameGraph {
            resources: Default::default(),
            graph: graph,
            top: top,
            bottom: bottom,
        }
    }

    pub fn add_pass<D: RenderPassDef<B>>(&mut self, def: &D) -> D::Output {
        let pass_ref = PassRef(self.graph.add_node(RenderStage::Invalid));
        let (output, pass) = {
            let mut builder = PassBuilder::new(pass_ref, self);
            def.setup_pass(&mut builder)
        };
        self.graph[pass_ref.0] = RenderStage::RenderPass(pass);
        output
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
            lifetime: ResourceLifetime {
                creator: self.pass,
                consumer: None,
            },
            name: name.to_string(),
        })
    }

    pub fn read_buffer(&mut self, buffer: &BufferRef) {
        self.frame.graph.add_edge(
            self.pass.0,
            self.frame.resources[buffer].lifetime.creator.0, 
            Dependency::Buffer(buffer.clone())
        );
    }

    pub fn write_buffer(&mut self, buffer: BufferRef, new_name: &str) -> BufferRef {
        self.read_buffer(&buffer);
        self.frame.resources.buffers[buffer.0].lifetime.consumer = Some(self.pass);
        let info = self.frame.resources.buffers[buffer.0].info;
        self.create_buffer(new_name, info)
    }

    pub fn create_image(&mut self, name: &str, info: ImageInfo) -> ImageRef {
        self.frame.resources.add_image(ImageResource {
            info: info,
            lifetime: ResourceLifetime {
                creator: self.pass,
                consumer: None,
            },
            name: name.to_string()
        })
    }

    pub fn read_image(&mut self, image: &ImageRef, layout: ImageLayout) {
        self.frame.graph.add_edge(
            self.pass.0,
            self.frame.resources[image].lifetime.creator.0, 
            Dependency::Image(image.clone(), layout)
        );
    }

    pub fn write_image(&mut self, image: ImageRef, new_name: &str, layout: ImageLayout) -> ImageRef {
        self.read_image(&image, layout);
        self.frame.resources.images[image.0].lifetime.consumer = Some(self.pass);
        let info = self.frame.resources.images[image.0].info;
        self.create_image(new_name, info)
    }
}