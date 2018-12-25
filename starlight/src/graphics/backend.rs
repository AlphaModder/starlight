#[cfg(all(feature = "dx12", target_os = "windows"))]
extern crate gfx_backend_dx12;
#[cfg(feature = "vulkan")]
extern crate gfx_backend_vulkan;
#[cfg(all(feature = "metal", target_os = "macos"))]
extern crate gfx_backend_metal;
#[cfg(feature = "gl")]
extern crate gfx_backend_gl;
use gfx_hal;
use gfx_hal::Adapter;
use gfx_hal::format::{self, AsFormat};
use winit;

pub trait Instance {
    type Backend: gfx_hal::Backend;
    type SurfaceError;
    fn create(engine_name: &str, version: u32) -> Self;
    fn create_surface(&self, window_builder: winit::WindowBuilder, events: &winit::EventsLoop) -> Result<<Self::Backend as gfx_hal::Backend>::Surface, Self::SurfaceError>;
    fn enumerate_adapters(&self) -> Vec<Adapter<Self::Backend>>;
}

pub trait Backend: gfx_hal::Backend {
    type Instance: Instance<Backend=Self>;
}

macro_rules! impl_nongl_backend {
    ($reexport_name:ident: $crate_name:ident) => {
        impl Instance for $crate_name::Instance {
            type Backend = $crate_name::Backend;
            type SurfaceError = winit::CreationError;
            fn create(engine_name: &str, version: u32) -> Self {
                $crate_name::Instance::create(engine_name, version)
            }

            fn create_surface(&self, window_builder: winit::WindowBuilder, events: &winit::EventsLoop) -> Result<<Self::Backend as gfx_hal::Backend>::Surface, Self::SurfaceError> {
                Ok(self.create_surface(&window_builder.build(&events)?))
            }

            fn enumerate_adapters(&self) -> Vec<Adapter<Self::Backend>> {
                <gfx_hal::Instance<Backend=Self::Backend>>::enumerate_adapters(self)
            }
        }

        impl Backend for $crate_name::Backend {
            type Instance = $crate_name::Instance;
        }

        pub type $reexport_name = $crate_name::Backend;
    }
}

#[cfg(all(feature = "dx12", target_os = "windows"))]
impl_nongl_backend!(DX12Backend: gfx_backend_dx12);

#[cfg(feature = "vulkan")]
impl_nongl_backend!(VulkanBackend: gfx_backend_vulkan);

#[cfg(all(feature = "metal", target_os = "macos"))]
impl_nongl_backend!(MetalBackend: gfx_backend_metal);

#[cfg(feature = "gl")]
impl Instance for gfx_backend_gl::Headless {
    type Backend = gfx_backend_gl::Backend;
    type SurfaceError = gfx_backend_gl::glutin::CreationError;
    fn create(engine_name: &str, version: u32) -> Self {
        let context = gfx_backend_gl::glutin::HeadlessRendererBuilder::new(1, 1)
            .build()
            .expect("Failed to create headless context!");
        gfx_backend_gl::Headless(context)
    }

    fn create_surface(&self, window_builder: winit::WindowBuilder, events: &winit::EventsLoop) -> Result<<Self::Backend as gfx_hal::Backend>::Surface, Self::SurfaceError> {
        let context_builder = gfx_backend_gl::config_context(
            gfx_backend_gl::glutin::ContextBuilder::new(),
            format::Rgba8Srgb::SELF,
            None,
        ).with_vsync(true);
        Ok(gfx_backend_gl::Surface::from_window(
            gfx_backend_gl::glutin::GlWindow::new(window_builder, context_builder, &events)?
        ))
    }

    fn enumerate_adapters(&self) -> Vec<Adapter<Self::Backend>> {
        <gfx_hal::Instance<Backend=Self::Backend>>::enumerate_adapters(self)
    }
}

#[cfg(feature = "gl")]
impl Backend for gfx_backend_gl::Backend {
    type Instance = gfx_backend_gl::Headless;
}

#[cfg(feature = "gl")]
pub type GLBackend = gfx_backend_gl::Backend;
