use futures::prelude::*;
use futures::stream;

use graphics::frame::graph;
use graphics::frame::{GraphicsPass, ComputePass, RenderContext};
use graphics::frame::pipeline::*;
use graphics::frame::pipeline::resources::ResourcesWrapper;

use util;

use gfx_hal::{Backend, Device};
use gfx_hal::{queue, command, pool};

use std::collections::HashMap;
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;

use thread_local::ThreadLocal;

use typemap;

enum PassData<'c, B: Backend> {
    Graphics {
        pass: &'c GraphicsPass<B>,
        queue: (&'c queue::QueueGroup<B, queue::Graphics>, usize),
    },
    Compute {
        pass: &'c ComputePass<B>,
        queue: (&'c queue::QueueGroup<B, queue::Compute>, usize)
    }
}

pub struct RecordCommandBuffers<'c, B: Backend> {
    passes: Vec<PassData<'c, B>>,
    recorder: ParallelCommandRecorder<'c, B>,
    resources: ResourcesWrapper<'c, B>,
}

impl<'c, B: Backend> RecordCommandBuffers<'c, B> {
    pub fn new(context: &'c RenderContext<'c, B>) -> Self {
        let passes = Self::load_passes(context.graph);
        RecordCommandBuffers {
            recorder: ParallelCommandRecorder::new(context.device, pool::CommandPoolCreateFlags::empty(), passes.len()),
            passes: passes,
            resources: ResourcesWrapper::new(context),
        }
    }
    
    fn load_passes(graph: &'c graph::FrameGraph<'c, B>) -> Vec<PassData<'c, B>> {
        unimplemented!()
    }
}

#[async_stream(item = SubmitWrapper<B>)]
fn execute_stream<'a, B: Backend>(this: &'a RecordCommandBuffers<'a, B>) -> Result<(), ()> {
    #[async]
    for pass in stream::iter_ok(this.passes.iter()) {
        stream_yield!(match pass {
            PassData::Graphics { ref pass, queue: (ref queue, _) } => {
                let mut pool = this.recorder.get_pool(queue);
                let mut command_buffer = pool.acquire_command_buffer(false);
                pass.draw(&mut GraphicsContext::new(&this.resources, &mut command_buffer));
                command_buffer.finish().into()
            }
            PassData::Compute { ref pass, queue: (ref queue, _) } => {
                let mut pool = this.recorder.get_pool(queue);
                let mut command_buffer = pool.acquire_command_buffer(false);
                pass.execute(&mut ComputeContext::new(&this.resources, &mut command_buffer));
                command_buffer.finish().into()
            }
        });
    }
    Ok(())
}

impl<'c, B: Backend> PipelineStage for RecordCommandBuffers<'c, B> {
    type Input = ();
    type Output = Vec<SubmitWrapper<B>>;
    type Error = ();

    #[async(boxed)]
    fn execute(&self, _: ()) -> Result<Self::Output, ()> {
        let mut submits = Vec::new();
        #[async]
        for submit in execute_stream(self) {
            submits.push(submit);
        }
        Ok(submits)
    }
}

type Submit<B, C> = command::Submit<B, C, command::OneShot, command::Primary>;

pub enum SubmitWrapper<B: Backend> {
    Graphics(Submit<B, queue::Graphics>),
    Compute(Submit<B, queue::Compute>),
}

impl<B: Backend> From<Submit<B, queue::Graphics>> for SubmitWrapper<B> {
    fn from(submit: Submit<B, queue::Graphics>) -> SubmitWrapper<B> {
        SubmitWrapper::Graphics(submit)
    }
}

impl<B: Backend> From<Submit<B, queue::Compute>> for SubmitWrapper<B> {
    fn from(submit: Submit<B, queue::Compute>) -> SubmitWrapper<B> {
        SubmitWrapper::Compute(submit)
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

    fn get_pool<'a, C: queue::Capability + 'static + Send>(&'a self, queue_group: &queue::QueueGroup<B, C>) -> RefMut<'a, pool::CommandPool<B, C>> {
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

struct _Capability<B: Backend, C: 'static + Send>(PhantomData<(B, C)>);
impl<B: Backend, C: 'static + Send> typemap::Key for _Capability<B, C> {
    type Value = HashMap<queue::QueueFamilyId, pool::CommandPool<B, C>>;
}