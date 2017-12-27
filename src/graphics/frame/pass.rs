use gfx_hal::Backend;
use graphics::frame::{RenderContext, PassBuilder};

pub trait RenderPass<B: Backend> {
    fn render(&self, &mut RenderContext<B>);
}

impl<B: Backend> RenderPass<B> for Fn(&mut RenderContext<B>) {
    fn render(&self, context: &mut RenderContext<B>) { self(context) }
} 

pub trait RenderPassDef<'d, B: Backend> {
    type Output;
    fn setup_pass(&self, &mut PassBuilder<'d>) -> (Self::Output, Box<RenderPass<B>>);
}

impl<'d, B: Backend, O> RenderPassDef<'d, B> for Fn(&mut PassBuilder<'d>) -> (O, Box<RenderPass<B>>) {
    type Output = O;
    fn setup_pass(&self, builder: &mut PassBuilder<'d>) -> (O, Box<RenderPass<B>>) { self(builder) }
}