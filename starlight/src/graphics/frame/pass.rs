use gfx_hal::Backend;
use super::context::*;
use super::graph::PassBuilder;

pub trait GraphicsPass<B: Backend> {
    fn execute(&self, &mut GraphicsContext<B>);
}

impl<B: Backend> GraphicsPass<B> for Fn(&mut GraphicsContext<B>) {
    fn execute(&self, context: &mut GraphicsContext<B>) { self(context) }
}

pub trait GraphicsPassDef<B: Backend> {
    type Output;
    fn setup_pass<'r, 'f>(&self, &mut PassBuilder<'r, 'f, B>) -> (Self::Output, Box<GraphicsPass<B>>);
}

impl<B: Backend, F, O> GraphicsPassDef<B> for F 
    where for<'r, 'f> F: Fn(&mut PassBuilder<'r, 'f, B>) -> (O, Box<GraphicsPass<B>>)
{
    type Output = O;
    fn setup_pass<'r, 'f>(&self, builder: &mut PassBuilder<'r, 'f, B>) -> (O, Box<GraphicsPass<B>>) { self(builder) }
}

pub trait ComputePass<B: Backend> {
    fn execute(&self, &mut ComputeContext<B>);
}

impl<B: Backend> ComputePass<B> for Fn(&mut ComputeContext<B>) {
    fn execute(&self, context: &mut ComputeContext<B>) { self(context) }
}

pub trait ComputePassDef<B: Backend> {
    type Output;
    fn setup_pass<'r, 'f>(&self, &mut PassBuilder<'r, 'f, B>) -> (Self::Output, Box<ComputePass<B>>);
}

impl<B: Backend, F, O> ComputePassDef<B> for F 
    where for<'r, 'f> F: Fn(&mut PassBuilder<'r, 'f, B>) -> (O, Box<ComputePass<B>>)
{
    type Output = O;
    fn setup_pass<'r, 'f>(&self, builder: &mut PassBuilder<'r, 'f, B>) -> (O, Box<ComputePass<B>>) { self(builder) }
}
