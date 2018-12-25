use futures::prelude::*;
use futures::prelude::{async, await};

use std::boxed::PinBox;

mod context;
mod record;
mod pump;

pub(crate) mod resources;

pub use self::context::*;
pub use self::record::*;
pub use self::pump::*;

pub trait PipelineStage {
    type Input;
    type Output;
    type Error;

    fn execute<'a>(&'a self, input: Self::Input) -> PinBox<Future<Item = Self::Output, Error = Self::Error> + 'a>;
}

impl<T: PipelineStage + ?Sized> PipelineStage for Box<T> {
    type Input = T::Input;
    type Output = T::Output;
    type Error = T::Error;

    fn execute<'a>(&'a self, input: Self::Input) -> PinBox<Future<Item = Self::Output, Error = Self::Error> + 'a> {
        (self as &T).execute(input)
    }
}

pub struct ChainStage<S1: PipelineStage, S2: PipelineStage<Input=S1::Output>>(S1, S2) where S2::Error: Into<S1::Error>;

impl<S1, S2> PipelineStage for ChainStage<S1, S2> 
    where S1: PipelineStage, S2: PipelineStage<Input=S1::Output>, S2::Error: Into<S1::Error> 
{
    type Input = S1::Input;
    type Output = S2::Output;
    type Error = S1::Error;

    #[async(boxed)]
    fn execute(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        await!(self.1.execute(await!(self.0.execute(input))?)).map_err(Into::into)
    }
}

pub struct FramePump<S: PipelineStage>(S);

impl<S: PipelineStage> FramePump<S> {
    pub fn new(stage: S) -> FramePump<S> 
    {
        FramePump(stage)
    }

    pub fn add_stage<S2>(self, s: S2) -> FramePump<ChainStage<S, S2>>
        where S2: PipelineStage<Input=S::Output>, S2::Error: Into<S::Error>
    {
        FramePump(ChainStage(self.0, s))
    }

    pub fn run(&mut self, start: S::Input, max_delay: usize) {
        let frames = [];
        
    }
}