use gfx_hal::Backend;
use graphics::frame::Resources;
use graphics::frame::{GraphicsContext, ComputeContext};


pub trait RenderPass<B: Backend> {
    type Context;
    type Resources;
    fn acquire_resources(&self, resources: &Resources<B>) -> Self::Resources;
    fn execute(&self, &mut Self::Context, resources: &Self::Resources);
}

pub trait AnyPass<C> {
    fn acquire_resources(&mut self);
    fn execute_pass(&mut self, context: &mut C);
}

pub trait AnyPassOwned<C> {
    fn mirror<'a>(&'a self) -> Box<AnyPass<C> + 'a>;
}

impl<B: Backend, P: RenderPass<B>> AnyPassOwned<P::Context> for P {
    fn mirror<'a>(&'a self) -> Box<AnyPass<P::Context> + 'a> {
        struct PackagedRenderPass<'a, B: Backend, P: RenderPass<B>> {
            pass: &'a P,
            resources: Option<P::Resources>,
        }

        impl<'a, B: Backend, P: RenderPass<B>> AnyPass<P::Context> for PackagedRenderPass<'a, B, P> {
            fn acquire_resources(&mut self) {
                if self.resources.is_none() {
                    self.resources = Some(self.pass.acquire_resources());
                }
            }

            fn execute_pass(&mut self, context: &mut P::Context) {
                self.acquire_resources();
                self.pass.execute_pass(context, &self.resources.unwrap());
            }
        }

        Box::new(PackagedRenderPass {
            pass: &self.pass,
            resources: None
        })
    }
}
