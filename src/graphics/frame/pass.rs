use gfx_hal::Backend;
use super::{RenderContext};
use super::graph::PassBuilder;

pub trait RenderPass<B: Backend> {
    fn render(&self, &mut RenderContext<B>);
}

impl<B: Backend> RenderPass<B> for Fn(&mut RenderContext<B>) {
    fn render(&self, context: &mut RenderContext<B>) { self(context) }
} 

pub(crate) struct CombinedRenderPass<'f, B: Backend>(Box<RenderPass<B> + 'f>, Box<RenderPass<B> + 'f>);

impl<'f, B: Backend> RenderPass<B> for CombinedRenderPass<'f, B> {
    fn render(&self, context: &mut RenderContext<B>) {
        self.0.render(context);
        self.1.render(context);
    }
}

pub trait RenderPassDef<B: Backend> {
    type Output;
    fn setup_pass<'r, 'f>(&self, &mut PassBuilder<'r, 'f, B>) -> (Self::Output, Box<RenderPass<B>>);
}

impl<B: Backend, F, O> RenderPassDef<B> for F 
    where for<'r, 'f> F: Fn(&mut PassBuilder<'r, 'f, B>) -> (O, Box<RenderPass<B>>)
{
    type Output = O;
    fn setup_pass<'r, 'f>(&self, builder: &mut PassBuilder<'r, 'f, B>) -> (O, Box<RenderPass<B>>) { self(builder) }
}

