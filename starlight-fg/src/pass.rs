use gfx_hal::Backend;
use std::marker::PhantomData;

pub struct GraphicsContext<B: Backend>(PhantomData<B>);
pub struct ComputeContext<B: Backend>(PhantomData<B>);

pub trait GraphicsPass<B: Backend> {
    fn draw(&self, context: &mut GraphicsContext<B>);
}

impl<B: Backend, T> GraphicsPass<B> for T
    where T: Fn(&mut GraphicsContext<B>)
{
    fn draw(&self, context: &mut GraphicsContext<B>) { self(context) }
}

pub trait ComputePass<B: Backend> {
    fn execute(&self, context: &mut ComputeContext<B>);
}

impl<B: Backend, T> ComputePass<B> for T
    where T: Fn(&mut ComputeContext<B>)
{
    fn execute(&self, context: &mut ComputeContext<B>) { self(context) }
}