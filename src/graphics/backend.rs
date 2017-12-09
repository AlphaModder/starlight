#[cfg(all(feature = "dx12", target_os = "windows"))]
extern crate gfx_backend_dx12;
#[cfg(feature = "vulkan")]
extern crate gfx_backend_vulkan;
#[cfg(all(feature = "metal", target_os = "macos"))]
extern crate gfx_backend_metal;
#[cfg(feature = "gl")]
extern crate gfx_backend_gl;

use std::fmt::Debug;
use gfx_hal;
use gfx_hal::{Instance, Adapter};
use gfx_hal::format;
use gfx_hal::format::Formatted;
use winit;


pub trait Backend: Sized {
    type WindowError: Debug;
    type GfxBackend: gfx_hal::Backend;

    fn init(engine_name: &str, version: u32, window_builder: winit::WindowBuilder, events: &winit::EventsLoop) -> Result<Self, Self::WindowError>;
    fn get_surface(&self) -> &<Self::GfxBackend as gfx_hal::Backend>::Surface;
    fn get_surface_mut(&mut self) -> &mut <Self::GfxBackend as gfx_hal::Backend>::Surface;
    fn enumerate_adapters(&self) -> Vec<Adapter<Self::GfxBackend>>;
}

macro_rules! impl_nongl_backend {
    ($name:ident: $crate_name:ident) => {
        pub struct $name($crate_name::Instance, winit::Window, <$crate_name::Backend as gfx_hal::Backend>::Surface);
        impl Backend for $name {
            type WindowError = winit::CreationError;
            type GfxBackend = $crate_name::Backend;

            fn init(engine_name: &str, version: u32, window_builder: winit::WindowBuilder, events: &winit::EventsLoop) -> Result<Self, Self::WindowError> {
                let instance = $crate_name::Instance::create(engine_name, version);
                let window = window_builder.build(&events)?;
                let surface = instance.create_surface(&window);
                Ok($name(instance, window, surface))
            }

            fn get_surface(&self) -> &<Self::GfxBackend as gfx_hal::Backend>::Surface {
                &self.2
            }

            fn get_surface_mut(&mut self) -> &mut <Self::GfxBackend as gfx_hal::Backend>::Surface {
                &mut self.2
            }

            fn enumerate_adapters(&self) -> Vec<Adapter<Self::GfxBackend>> {
                self.0.enumerate_adapters()
            }
        }
    }
}

#[cfg(all(feature = "dx12", target_os = "windows"))]
impl_nongl_backend!(DX12: gfx_backend_dx12);

#[cfg(feature = "vulkan")]
impl_nongl_backend!(Vulkan: gfx_backend_vulkan);

#[cfg(all(feature = "metal", target_os = "macos"))]
impl_nongl_backend!(Metal: gfx_backend_metal);

#[cfg(feature = "gl")]
pub struct GL(<gfx_backend_gl::Backend as gfx_hal::Backend>::Surface);

#[cfg(feature = "gl")]
impl Backend for GL {
    type WindowError = gfx_backend_gl::glutin::CreationError;
    type GfxBackend = gfx_backend_gl::Backend;

    fn init(engine_name: &str, version: u32, window_builder: winit::WindowBuilder, events: &winit::EventsLoop) -> Result<Self, Self::WindowError> {
        let context_builder = gfx_backend_gl::config_context(
            gfx_backend_gl::glutin::ContextBuilder::new(),
            format::Srgba8::SELF,
            None,
        ).with_vsync(true);
        Ok(GL(gfx_backend_gl::Surface::from_window(
            gfx_backend_gl::glutin::GlWindow::new(window_builder, context_builder, &events)?
        )))
    }

    fn get_surface(&self) -> &<Self::GfxBackend as gfx_hal::Backend>::Surface {
        &self.0
    }

    fn get_surface_mut(&mut self) -> &mut <Self::GfxBackend as gfx_hal::Backend>::Surface {
        &mut self.0
    }

    fn enumerate_adapters(&self) -> Vec<Adapter<Self::GfxBackend>> {
        self.0.enumerate_adapters()
    }
}