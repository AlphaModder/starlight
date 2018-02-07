use gfx_hal::Backend;

pub trait RenderPass<B: Backend> {
    type Context;
    type Resources;
    fn execute(&self, &mut Self::Context, resources: &Self::Resources);
    fn acquire_resources(&self) -> Self::Resources;
}

/*
impl<B: Backend> GraphicsPass<B> for Fn(&mut GraphicsContext<B>) {
    fn execute(&self, context: &mut GraphicsContext<B>,) { self(context) }
}

impl<B: Backend, F, O> GraphicsPassDef<B> for F 
    where for<'r, 'f> F: Fn(&mut PassBuilder<'r, 'f, B>) -> (O, Box<GraphicsPass<B>>)
{
    type Output = O;
    fn setup_pass<'r, 'f>(&self, builder: &mut PassBuilder<'r, 'f, B>) -> (O, Box<GraphicsPass<B>>) { self(builder) }
}
*/
