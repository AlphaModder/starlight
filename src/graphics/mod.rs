use winit;
use gfx_hal;
use gfx_hal::{format, window};
use gfx_hal::{Device, Gpu, Adapter, Instance, QueueFamily, Surface};
use graphics::backend::Backend;

pub mod graph;
pub mod command;
pub mod resource;
pub mod backend;

pub struct Graphics<B: Backend> {
    backend: B,
    window_events: winit::EventsLoop,
    surface_format: format::Format,
    swapchain: <B::GfxBackend as gfx_hal::Backend>::Swapchain,
    backbuffer: window::Backbuffer<B::GfxBackend>,
    gpu: Gpu<B::GfxBackend>,
}

impl<B: Backend> Graphics<B> {
    pub(crate) fn new(window_builder: winit::WindowBuilder) -> Result<Graphics<B>, GraphicsInitError<B>> {
        info!("Initializing graphics subsystem...");
        let mut events = winit::EventsLoop::new();
        let mut backend = B::init("mvp engine", 1, window_builder, &events);
        let mut backend = {
            match backend {
                Ok(b) => b,
                Err(e) => return Err(GraphicsInitError::WindowCreationFailed(e))
            }
        };

        info!("Acquiring GPU...");
        let (gpu, surface_format) = Self::acquire_gpu_and_surface_format(&backend)?;
        
        let (mut swapchain, backbuffer) = gpu.device.create_swapchain(
            backend.get_surface_mut(),
            window::SwapchainConfig::new().with_color(surface_format),
        );

        Ok(Graphics {
            window_events: events,
            backend: backend,
            surface_format: surface_format,
            swapchain: swapchain,
            backbuffer: backbuffer,
            gpu: gpu,
        })
    }

    fn acquire_gpu_and_surface_format(backend: &B) -> Result<(Gpu<B::GfxBackend>, format::Format), GraphicsInitError<B>> {
        let mut adapters = backend.enumerate_adapters();
        let adapter = adapters.remove(0);
        debug!("Using adapter:");
        debug!("{:?}", adapter.info);
        
        let surface_format = {
            let _surface_format = Self::get_surface_format(&adapter, backend.get_surface()); 
            match _surface_format {
                Some(f) => f,
                None => return Err(GraphicsInitError::NoCompatibleSurfaceFormat),
            }
        };

        debug!("{:?}", surface_format);

        Ok((adapter.open_with(|ref family| {
            if family.supports_graphics() && backend.get_surface().supports_queue_family(family) {
                Some(1)
            } else { None }
        }), surface_format))
    }

    fn get_surface_format(adapter: &Adapter<B::GfxBackend>, surface: &Surface<B::GfxBackend>) -> Option<format::Format> {
        surface
            .capabilities_and_formats(&adapter.physical_device)
            .1
            .into_iter()
            .find(|format| format.1 == format::ChannelType::Srgb)
    }
}

#[derive(Debug)]
pub enum GraphicsInitError<B: Backend> {
    WindowCreationFailed(B::WindowError),
    NoCompatibleSurfaceFormat,
}



  