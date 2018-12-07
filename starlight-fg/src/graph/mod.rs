use daggy::{self, NodeIndex};

use gfx_hal::Backend;

mod build;
mod resources;
pub mod internal;

pub use self::build::*;
pub use self::resources::*;
use self::internal::*;

#[derive(Eq, PartialEq)]
pub struct PassRef(NodeIndex);

#[derive(Eq, PartialEq)]
pub struct BufferRef(PassRef, usize);

#[derive(Eq, PartialEq)]
pub struct ImageRef(PassRef, usize);


pub struct FrameGraph<'p, B: Backend> {
    graph: daggy::Dag<Option<RenderPass<'p, B>>, PassDependency>,
}

impl<'p, B: Backend> Default for FrameGraph<'p, B> {
    fn default() -> FrameGraph<'p, B> {
        FrameGraph {
            graph: Default::default(),
        }
    }
}

impl<'p, B: Backend> FrameGraph<'p, B> {
    pub fn new() -> FrameGraph<'p, B> {
        Default::default()
    }

    pub fn add_graphics_pass<T: BuildGraphicsPass<B>>(&mut self, build: T) -> T::Output
        where T::Pass: 'p, 
    {
        let pass_ref = PassRef(self.graph.add_node(None));
        let mut builder = GraphicsPassBuilder::new(self, pass_ref);
        let (output, pass) = build.build(&mut builder);
        *self.graph.node_weight_mut(pass_ref.0).unwrap() = Some(RenderPass {
            buffers: builder.buffers,
            images: builder.images,
            kind: RenderPassKind::Graphics(Box::new(pass))
        });
        output
    }

    pub fn add_compute_pass<T: BuildComputePass<B>>(&mut self, build: T) -> T::Output
        where T::Pass: 'p,
    {
        let pass_ref = PassRef(self.graph.add_node(None));
        let mut builder = ComputePassBuilder::new(self, pass_ref);
        let (output, pass) = build.build(&mut builder);
        *self.graph.node_weight_mut(pass_ref.0).unwrap() = Some(RenderPass {
            buffers: builder.buffers,
            images: builder.images,
            kind: RenderPassKind::Compute(Box::new(pass))
        });
        output
    }

    
}

impl<'p, B: Backend> FrameGraph<'p, B> {
    
}

