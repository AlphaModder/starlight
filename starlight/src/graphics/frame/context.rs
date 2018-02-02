use gfx_hal::Backend;
use std::marker::PhantomData;

pub struct GraphicsContext<B: Backend> {
    phantom: PhantomData<B>,
}

pub struct ComputeContext<B: Backend> {
    phantom: PhantomData<B>,
}