use gfx_hal::{Backend, /* QueueFamily */};
// use gfx_hal::{backend, queue};
use graphics::frame::graph;

pub struct RenderContext<'c, B: Backend> {
    //queues: Vec<(&'c B::Device, backend::RawQueueGroup<B>)>,
    pub(crate) device: &'c B::Device,
    pub(crate) graph: &'c graph::FrameGraph<'c, B>,
}

/*
impl<'c, B: Backend> RenderContext<'c, B> {
    pub fn get_queue_groups<C: queue::Capability>(&self) -> impl Iterator<Item=QueueGroup<B, C>> {
        self.queues.iter()
            .filter(|(_, group)| C::supported_by(group.family.queue_type()))
            .map(|(device, group)| QueueGroup { device: *device, group: group })
    }
}


pub struct QueueGroup<'q, B: Backend, C> {
    device: &'q B::Device,
    group: &'q backend::RawQueueGroup<B>,
}

impl<'q, B: Backend, C> QueueGroup<'q, B, C> {
    pub fn get_queues(&self) -> impl Iterator<Item=CommandQueue<B, C>> {
        self.group.queues.iter().map(|queue| CommandQueue { device: self.device, queue: queue })
    }
}

pub struct CommandQueue<'q, B: Backend, C> {
    device: &'q B::Device,
    queue: &'q B::CommandQueue,
}
*/