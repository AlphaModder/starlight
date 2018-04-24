use futures::prelude::*;
use futures::future;

use graphics::frame::graph;
use graphics::frame::pipeline::*;
use graphics::frame::pass::{GraphicsPass, ComputePass};

use util;

use gfx_hal::{Backend, Device};
use gfx_hal::{queue, command, pool};

use std::collections::HashMap;
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;

use thread_local::ThreadLocal;

use typemap;

pub struct RecordCommandBuffers<'c, B: Backend> {
    context: &'c RenderContext<'c, B>,
    passes: Vec<&'c graph::RenderPass<'c, B>>,
    recorder: ParallelCommandRecorder<'c, B>,
}

impl<'c, B: Backend> RecordCommandBuffers<'c, B> {
    fn reorder_passes(graph: &'c graph::FrameGraph<'c, B>) -> Vec<&'c graph::RenderPass<'c, B>> {
        unimplemented!()
    }
}

impl<'c, B: Backend> PipelineStage<'c, B> for RecordCommandBuffers<'c, B> {
    type Data = ();
    type Input = ();
    type Output = Vec<SubmitWrapper<B>>;
    type Error = ();

    fn new(context: &'c RenderContext<'c, B>, data: &()) -> Self {
        let passes = Self::reorder_passes(context.graph);
        RecordCommandBuffers {
            context: context,
            passes: passes,
            recorder: ParallelCommandRecorder::new(context.device, pool::CommandPoolCreateFlags::empty(), passes.len()),
        }
    }

    fn execute<'a>(&'a mut self, input: ()) -> Box<Future<Item=Vec<SubmitWrapper<B>>, Error=()> + 'a> {
        let recorder = &self.recorder;
        Box::new(self.passes.iter().filter_map(move |pass| {
            match **pass {
                graph::RenderPass::Graphics(ref pass) => Some(RecordCommandBufferWrapper::Graphics(
                    RecordCommandBuffer {
                        record: &|command_buffer| {
                            let context = GraphicsContextImpl::new(unimplemented!(), command_buffer);
                            pass.draw(&mut context)
                        },
                        queue: unimplemented!(),
                        recorder: recorder,
                    }
                )),
                graph::RenderPass::Compute(ref pass) => Some(RecordCommandBufferWrapper::Compute(
                    RecordCommandBuffer {
                        record: &|command_buffer| {
                            let context = ComputeContextImpl::new(unimplemented!(), command_buffer);
                            pass.execute(&mut context)
                        },
                        queue: unimplemented!(),
                        recorder: recorder,
                    }
                )),
                _ => None,
            }
        }).collect::<future::JoinAll<_>>())
    }
}

type Submit<B, C> = command::Submit<B, C, command::OneShot, command::Primary>;

enum RecordCommandBufferWrapper<'f, B: Backend> {
    Graphics(RecordCommandBuffer<'f, B, queue::Graphics>),
    Compute(RecordCommandBuffer<'f, B, queue::Compute>),
}

pub enum SubmitWrapper<B: Backend> {
    Graphics(Submit<B, queue::Graphics>),
    Compute(Submit<B, queue::Compute>),
}

impl<'f, B: Backend> Future for RecordCommandBufferWrapper<'f, B> {
    type Item = SubmitWrapper<B>;
    type Error = ();

    fn poll(&mut self, context: &mut Context) -> Result<Async<Self::Item>, Self::Error> {
        match *self {
            RecordCommandBufferWrapper::Graphics(ref mut inner) => inner.poll(context).map(|a| a.map(|s| SubmitWrapper::Graphics(s))),
            RecordCommandBufferWrapper::Compute(ref mut inner) => inner.poll(context).map(|a| a.map(|s| SubmitWrapper::Compute(s))),
        }
    }
}

struct RecordCommandBuffer<'f, B: Backend, C: queue::Capability + 'f> {
    record: &'f for<'b> Fn(&mut command::CommandBuffer<'b, B, C>),
    queue: &'f queue::QueueGroup<B, C>,
    recorder: &'f ParallelCommandRecorder<'f, B>
}

impl<'f, B: Backend, C: queue::Capability + 'f> Future for RecordCommandBuffer<'f, B, C> {
    type Item = Submit<B, C>;
    type Error = ();

    fn poll(&mut self, context: &mut Context) -> Result<Async<Self::Item>, Self::Error> {
        let mut pool = self.recorder.get_pool(self.queue);
        let mut command_buffer = pool.acquire_command_buffer(false);
        (self.record)(&mut command_buffer);
        Ok(Async::Ready(command_buffer.finish()))
    }
}

struct ParallelCommandRecorder<'c, B: Backend> {
    device: &'c B::Device,
    pools: ThreadLocal<RefCell<typemap::SendMap>>,
    flags: pool::CommandPoolCreateFlags,
    capacity: usize
}

impl<'c, B: Backend> ParallelCommandRecorder<'c, B> {
    fn new(device: &B::Device, flags: pool::CommandPoolCreateFlags, capacity: usize) -> ParallelCommandRecorder<B> { 
        ParallelCommandRecorder { 
            device: device,
            pools: ThreadLocal::new(),
            flags: flags,
            capacity: capacity,
        }
    }

    fn get_pool<'a, C: queue::Capability>(&'a self, queue_group: &queue::QueueGroup<B, C>) -> RefMut<'a, pool::CommandPool<B, C>> {
        RefMut::map(
            self.pools.get_or(|| Box::new(RefCell::new(typemap::SendMap::custom()))).borrow_mut(),
            |map| {
                unsafe {
                    ::std::mem::transmute(map.entry::<_Capability<B, C>>().or_insert_with(
                        || util::transmute_unchecked(self.device.create_command_pool_typed(queue_group, self.flags, self.capacity)
                    )))
                }
            }
        )
    }
}

struct _Capability<B: Backend, C>(PhantomData<(B, C)>);
impl<B: Backend, C> typemap::Key for _Capability<B, C> {
    type Value = HashMap<queue::QueueFamilyId, pool::CommandPool<B, C>>;
}
