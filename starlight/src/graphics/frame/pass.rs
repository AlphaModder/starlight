use gfx_hal::Backend;
use graphics::frame::pipeline::{GraphicsContext, ComputeContext};

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