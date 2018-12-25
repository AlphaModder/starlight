#![feature(proc_macro, generators, pin)]

extern crate futures;
use futures::prelude::*;
use futures::prelude::{async, await};
use futures::task::Context;
use futures::executor::Executor;
use futures::channel::mpsc;
use std::boxed::PinBox;

struct FramePump {

}

trait PipelineStage {
    type Input;
    type Error;
    fn execute<'a>(&'a mut self, input: Self::Input) -> PinBox<Future<Item=(), Error=Self::Error>>;
}

pub struct StageTask<S: PipelineStage> {
    stage: S,
    reciever: mpsc::UnboundedReceiver<S::Input>,
}

impl<S: PipelineStage> StageTask<S> {
    #[async]
    fn run(&mut self) -> Result<(), S::Error> {
        loop {
            match await_item!(self.reciever).unwrap() {
                Some(item) => await!(self.stage.execute(item))?,
                None => break,
            }
        }
        Ok(())
    }
}

pub struct StageHandle<I> {
    sender: mpsc::UnboundedSender<I>,
}

impl<I> StageHandle<I> {
    pub fn 
}

impl<I> Sink for StageHandle<I> {
    type SinkItem = I;
    type SinkError = mpsc::SendError;

    fn poll_ready(&mut self, context: &mut Context) -> Result<Async<()>, Self::SinkError> {
        self.sender.poll_ready(context)
    }

    fn start_send(&mut self, item: Self::SinkItem) -> Result<(), Self::SinkError> {
        self.sender.start_send(item)
    }

    fn poll_flush(&mut self, context: &mut Context) -> Result<Async<()>, Self::SinkError> {
        self.sender.poll_flush(context)
    }

    fn poll_close(&mut self, context: &mut Context) -> Result<Async<()>, Self::SinkError> {
        self.sender.poll_close(context)
    }
}

impl FramePump {

    pub fn add_stage<S: PipelineStage>(&self, stage: S) -> StageHandle<S::Input> {

    }

    fn start(&self, executor: &Executor) {

    }
}

macro_rules! frame_pump {
    {
        pump $name:ident {
            $($stage:tt),+
        }
    } => {
        struct $name {

        }

        mod queues {
            $(stage_queues!{$name, $stage})*   
        }

        mod traits {
            trait Stage { 
                fn execute(&self, pump: &$name);
            }

            trait Produces<T> {
                fn produce(&self);
            }
        }

        $(stage_definition!{$name, $stage})*   
    }
}

macro_rules! stage_queues {
    {
        $name:ident, $stage:ident {
            &self, $($field:ident: $type:ty),+
        } -> ($($result_ty:ty),+) $body:block
    } => {
        struct $stage($(StageQueue<$result_ty>),+);
    }
}

macro_rules! stage_definition {
    {
        $name:ident, $stage:ident {
            &self, $($field:ident: $type:ty),+
        } -> ($($result_ty:ty),+) $body:block
    } => {
        struct $stage {
            $($field:ident: $type:ty),+
        }

        impl traits::Stage for $stage {
            fn execute(&self, pump: &$name) {
                $(let $field = pump.pull::<$type>();),+
                $body
            }
        }

        $(impl traits::Produces<$result_ty> for $name {
            fn produce(pump: &$name) {

            }
        })+
    }
}

