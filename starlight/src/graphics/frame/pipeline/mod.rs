use futures::prelude::*;
use futures::task::Context;

use gfx_hal::Backend;

use graphics::frame::graph;
use util::ResourcePool;

mod record;
mod resources;
mod context;

use self::context::*;

pub use self::resources::*;
pub use self::record::*;
pub use self::context::{GraphicsContext, ComputeContext};

pub struct RenderContext<'c, B: Backend> {
    graph: &'c graph::FrameGraph<'c, B>,
    device: &'c B::Device,
}

pub trait PipelineStage<'c, B: Backend> {
    type Data;
    type Input;
    type Output;
    type Error;

    fn new(graph: &'c RenderContext<'c, B>, data: &Self::Data) -> Self;

    fn execute<'a>(&'a mut self, input: Self::Input) -> Box<Future<Item = Self::Output, Error = Self::Error> + 'a>;
}

pub struct FrameFuture<'f, O, E>(Box<Future<Item=O, Error=E> + 'f>);

impl<'f, O, E> Future for FrameFuture<'f, O, E> {
    type Item = O;
    type Error = E;

    fn poll(&mut self, context: &mut Context) -> Result<Async<Self::Item>, Self::Error> {
        self.0.poll(context)
    }
}

pub struct FramePump<'c, B: Backend, I: 'c, O: 'c, E: 'c> {
    context: &'c RenderContext<'c, B>,
    future_builder: Box<Fn(I) -> Box<Future<Item=O, Error=E> + 'c> +'c>,
    max_frames: Option<usize>,
}

impl<'c, B: Backend, I: 'c, O: 'c, E: 'c> FramePump<'c, B, I, O, E> {
    pub fn new<S>(context: &'c RenderContext<'c, B>, max_frames: Option<usize>, data: S::Data, count: usize) -> FramePump<'c, B, I, O, E> 
        where S: PipelineStage<'c, B, Input=I, Output=O, Error=E> + 'c
    {
        let pool = ResourcePool::new(count.min(max_frames.unwrap_or(count)), || S::new(context, &data));
        FramePump {
            context: context,
            future_builder: Box::new(|input| {
                Box::new(pool.acquire().map_err(|e| panic!("Failed to acquire stage.")).and_then(|stage| stage.execute(input)))
            }),
            max_frames: max_frames,
        }
    }

    pub fn add_stage<S>(self, data: S::Data, count: usize) -> FramePump<'c, B, I, S::Output, E> 
        where S: PipelineStage<'c, B, Input=O> + 'c, S::Error: Into<E>
    {
        let pool = ResourcePool::new(count.min(self.max_frames.unwrap_or(count)), || S::new(self.context, &data));
        FramePump {
            context: self.context,
            max_frames: self.max_frames,
            future_builder: Box::new(move |input| {
                Box::new(
                    (self.future_builder)(input).and_then(
                        |output| {
                            pool.acquire()
                                .map_err(|e| panic!("Failed to acquire stage."))
                                .and_then(|stage| stage.execute(output))
                                .map_err(Into::into)
                        }
                    )
                )
            }),
        }
    }

    pub fn start_frame(&self, input: I) -> FrameFuture<O, E> {
        FrameFuture((self.future_builder)(input))
    }
}
