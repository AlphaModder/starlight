use crate::FrameGraph;
use crate::pass::{GraphicsPass, ComputePass};

mod graph {
    pub use crate::graph::*;
    pub use crate::graph::internal::{RenderPass, RenderPassKind, FrameGraphInternals};
}

use self::graph::*;

use gfx_hal::{Backend, Device};
use gfx_hal::pool::{RawCommandPool, CommandPoolCreateFlags};
use gfx_hal::command::{RawCommandBuffer, RawLevel, SubpassContents};
use gfx_hal::pso::Rect;

enum RenderPassKind<'p, B: Backend> {
    Graphics(&'p dyn GraphicsPass<B>),
    Compute(&'p dyn ComputePass<B>),
}

struct RenderPass<'p, B: Backend> {
    pass_handle: B::RenderPass,
    render_area: Rect,
    first_subpass: SubpassContents,
    kind: RenderPassKind<'p, B>,
}

impl<'p, B: Backend> RenderPass<'p, B> {
    fn record(&self, buffer: &mut B::CommandBuffer) {
        buffer.begin_render_pass(
            &self.pass_handle,
            unimplemented!(),
            self.render_area,
            unimplemented!(),
            self.first_subpass,
        );
    }
}

pub struct SerialRenderer<'g, B: Backend> {
    device: &'g B::Device,
    graph: &'g FrameGraph<'g, B>,
    graphics_pool: B::CommandPool,
    graphics_buffers: Vec<B::CommandBuffer>,
    compute_pool: B::CommandPool,
    compute_buffers: Vec<B::CommandBuffer>,
    passes: Vec<RenderPass<'g, B>>,
}

impl<'g, B: Backend> SerialRenderer<'g, B> {
    pub fn new(device: &'g B::Device, graph: &'g FrameGraph<'g, B>) -> Self {
        let graphics_pool = device.create_command_pool(unimplemented!(), CommandPoolCreateFlags::TRANSIENT);
        let compute_pool = device.create_command_pool(unimplemented!(), CommandPoolCreateFlags::TRANSIENT);
        let (graphics_buffers, compute_buffers) = Self::allocate_command_buffers(graph, compute_pool, graphics_pool);
        SerialRenderer {
            device: device,
            graph: graph,
            graphics_pool: graphics_pool,
            graphics_buffers: graphics_buffers,
            compute_pool: compute_pool,
            compute_buffers: compute_buffers,
            passes: Self::create_passes(device, graph),
        }
    }
}

impl<'g, B: Backend> SerialRenderer<'g, B> {
    fn allocate_command_buffers(
        graph: &'g FrameGraph<'g, B>, 
        graphics_pool: B::CommandPool, 
        compute_pool: B::CommandPool,
    ) -> (Vec<B::CommandBuffer>, Vec<B::CommandBuffer>) {
        let (graphics_buffers, compute_buffers) = graph.passes().fold((0, 0), |(g, c), p| match p.kind {
            graph::RenderPassKind::Graphics(_) => (g + 1, c),
            graph::RenderPassKind::Compute(_) => (g, c + 1),
        });
        (graphics_pool.allocate(graphics_buffers, RawLevel::Primary), compute_pool.allocate(compute_buffers, RawLevel::Primary))
    }

    fn create_passes(device: &'g B::Device, graph: &'g FrameGraph<'g, B>) -> Vec<RenderPass<'g, B>> {
        graph.passes().filter_map(|pass| {
            match pass.kind {
                graph::RenderPassKind::Graphics(ref pass) => {
                    Some(RenderPass {
                        pass_handle: unimplemented!(),
                        kind: RenderPassKind::Graphics(&pass),
                    })
                },
                graph::RenderPassKind::Compute(ref pass) => {
                    Some(RenderPass {
                        pass_handle: unimplemented!(),
                        kind: RenderPassKind::Compute(&pass),
                    })
                }
                _ => None
            }
        }).collect::<Vec<_>>()
    }
}

impl<'g, B: Backend> Drop for SerialRenderer<'g, B> {
    fn drop(&mut self) {
        self.graphics_pool.free(self.graphics_buffers);
        self.device.destroy_command_pool(self.graphics_pool);
        self.compute_pool.free(self.compute_buffers);
        self.device.destroy_command_pool(self.compute_pool);
    }
}