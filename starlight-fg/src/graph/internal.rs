use daggy::petgraph::{self, algo::{self, DfsSpace}};
use daggy::Walker;

use gfx_hal::Backend;

use crate::graph::{FrameGraph, PassRef, BufferRef, BufferResource, ImageRef, ImageResource, ImageRead};
use crate::pass::{GraphicsPass, ComputePass};

use self::PassDependency::*;

pub enum PassDependency {
    ReadBuffer(usize),
    ReadImage(usize, ImageRead),
    CopyBuffer(usize, usize),
    CopyImage(usize, usize),
}

pub struct RenderPass<'p, B: Backend> {
    buffers: Vec<BufferResource>,
    images: Vec<ImageResource>,
    pub kind: RenderPassKind<'p, B>
}

pub enum RenderPassKind<'p, B: Backend> {
    Graphics(Box<dyn GraphicsPass<B> + 'p>),
    Compute(Box<dyn ComputePass<B> + 'p>),
}

pub trait FrameGraphInternals<'p, B: Backend> {
    fn get_pass(&self, pass: PassRef) -> &RenderPass<B>;

    fn get_buffer(&self, buffer: BufferRef) -> &BufferResource {
        &self.get_pass(buffer.0).buffers[buffer.1]
    }

    fn get_image(&self, image: ImageRef) -> &ImageResource {
        &self.get_pass(image.0).images[image.1]
    }

    fn buffer_lifetime_overlap_forward(&self, b1: BufferRef, b2: BufferRef) -> bool;

    fn buffer_lifetime_overlap_backward(&self, b1: BufferRef, b2: BufferRef) -> bool {
        self.buffer_lifetime_overlap_forward(b2, b1)
    }

    fn image_lifetime_overlap_forward(&self, i1: ImageRef, i2: ImageRef) -> bool;

    fn image_lifetime_overlap_backward(&self, i1: ImageRef, i2: ImageRef) -> bool {
        self.image_lifetime_overlap_forward(i2, i1)
    }

    //TODO: Make these impl Iterator as soon as available in traits.

    fn pass_refs(&self) -> Box<dyn Iterator<Item=PassRef>>; 

    fn passes<'a>(&'a self) -> Box<dyn Iterator<Item=&'a RenderPass<'a, B>> + 'a>;
}

impl<'p, B: Backend> FrameGraphInternals<'p, B> for FrameGraph<'p, B> {
    fn get_pass(&self, pass: PassRef) -> &RenderPass<B> {
        &self.graph[pass.0].unwrap()
    }

    fn buffer_lifetime_overlap_forward(&self, b1: BufferRef, b2: BufferRef) -> bool {
        let dfs_space = DfsSpace::new(self.graph.graph());
        for (_, reader) in self.graph.children((b1.0).0)
            .iter(&self.graph)
            .filter(|(e, _)| { 
                match self.graph[*e] {
                    ReadBuffer(b) | CopyBuffer(b, _) if b == b1.1 => true,
                    _ => false,
                }
            }) 
        {
            if algo::has_path_connecting(self.graph.graph(), (b2.0).0, reader, Some(&mut dfs_space)) {
                return true
            }
        }
        false
    }

    fn image_lifetime_overlap_forward(&self, i1: ImageRef, i2: ImageRef) -> bool {
        let dfs_space = DfsSpace::new(self.graph.graph());
        for (_, reader) in self.graph.children((i1.0).0)
            .iter(&self.graph)
            .filter(|(e, _)| { 
                match self.graph[*e] {
                    ReadImage(i, _) | CopyImage(i, _) if i == i1.1 => true,
                    _ => false,
                }
            }) 
        {
            if algo::has_path_connecting(self.graph.graph(), (i2.0).0, reader, Some(&mut dfs_space)) {
                return true
            }
        }
        false
    }

    fn pass_refs(&self) -> Box<dyn Iterator<Item=PassRef>> {
        Box::new(self.graph.graph().node_indices().map(|i| PassRef(i)))
    }

    fn passes<'a>(&'a self) -> Box<dyn Iterator<Item=&'a RenderPass<'a, B>> + 'a> {
        Box::new(self.graph.graph().node_indices().map(|i| self.get_pass(PassRef(i))))
    }
}

pub(crate) trait FrameGraphInternalsMut<'p, B: Backend> {
    fn get_pass_mut(&mut self, pass: PassRef) -> &mut RenderPass<B>;

    fn get_buffer_mut(&mut self, buffer: BufferRef) -> &mut BufferResource {
        &mut self.get_pass_mut(buffer.0).buffers[buffer.1]
    }

    fn get_image_mut(&mut self, image: ImageRef) -> &mut ImageResource {
        &mut self.get_pass_mut(image.0).images[image.1]
    }
}

impl<'p, B: Backend> FrameGraphInternalsMut<'p, B> for FrameGraph<'p, B> {
    fn get_pass_mut(&mut self, pass: PassRef) -> &mut RenderPass<B> {
        &mut self.graph[pass.0].unwrap()
    }
}