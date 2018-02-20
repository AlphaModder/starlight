use winit;
use gfx_hal;
use gfx_hal::{format, window};
use gfx_hal::{Device, Adapter, Instance, Surface, QueueGroup};
use gfx_hal::error::DeviceCreationError;
use graphics::backend::Backend;

pub mod frame;
// pub mod frame_old;
pub mod backend;

pub struct Graphics<B: Backend> {
    backend: B,
    window_events: winit::EventsLoop,
    surface_format: format::Format,
    swapchain: <B::GfxBackend as gfx_hal::Backend>::Swapchain,
    backbuffer: window::Backbuffer<B::GfxBackend>,
    device: <B::GfxBackend as gfx_hal::Backend>::Device,
}

impl<B: Backend> Graphics<B> {
    pub(crate) fn new(window_builder: winit::WindowBuilder) -> Result<Graphics<B>, GraphicsInitError<B>> {
        info!("Initializing graphics subsystem...");
        let mut events = winit::EventsLoop::new();
        let mut backend = B::init("Starlight", 1, window_builder, &events);
        let mut backend = {
            match backend {
                Ok(b) => b,
                Err(e) => return Err(GraphicsInitError::WindowCreationFailed(e))
            }
        };

        info!("Acquiring GPU...");
        let DeviceData { device, queue_group, surface_format } = Self::acquire_device_data(&backend)?;
        
        let (mut swapchain, backbuffer) = device.create_swapchain(
            backend.get_surface_mut(),
            window::SwapchainConfig::new().with_color(surface_format),
        );

        Ok(Graphics {
            window_events: events,
            backend: backend,
            surface_format: surface_format,
            swapchain: swapchain,
            backbuffer: backbuffer,
            device: device,
        })
    }

    fn acquire_device_data(backend: &B) -> Result<DeviceData<B>, GraphicsInitError<B>> {
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

        let (device, queue_group) = adapter.open_with::<_, gfx_hal::Graphics>(1, |family| {
            backend.get_surface().supports_queue_family(family)
        })?;

        Ok(DeviceData {
            device: device, 
            queue_group: queue_group, 
            surface_format: surface_format
        })
    }

    fn get_surface_format(adapter: &Adapter<B::GfxBackend>, surface: &Surface<B::GfxBackend>) -> Option<format::Format> {
        surface.capabilities_and_formats(&adapter.physical_device).1.map(|formats| { 
                formats.into_iter().find(|format| {
                    format.base_format().1 == format::ChannelType::Srgb
                }).unwrap()
            }
        )
    }
}

struct DeviceData<B: Backend> {
    device: <B::GfxBackend as gfx_hal::Backend>::Device,
    queue_group: QueueGroup<B::GfxBackend, gfx_hal::Graphics>,
    surface_format: format::Format
}

#[derive(Debug)]
pub enum GraphicsInitError<B: Backend> {
    WindowCreationFailed(B::WindowError),
    DeviceCreationFailed(DeviceCreationError),
    NoCompatibleSurfaceFormat,
}

impl<B: Backend> From<DeviceCreationError> for GraphicsInitError<B> {
    fn from(error: DeviceCreationError) -> GraphicsInitError<B> {
        GraphicsInitError::DeviceCreationFailed(error)
    }
}

  