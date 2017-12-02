extern crate gfx_hal;
#[cfg(feature = "dx12")]
extern crate gfx_backend_dx12 as back;
#[cfg(feature = "vulkan")]
extern crate gfx_backend_vulkan as back;
#[cfg(feature = "metal")]
extern crate gfx_backend_metal as back;
#[cfg(feature = "gl")]
extern crate gfx_backend_gl as back;

use winit;
use self::gfx_hal::{format};
use self::gfx_hal::{Device, Gpu, Adapter, Instance, QueueFamily, Surface as _Surface};

#[cfg(any(feature = "vulkan", feature = "dx12", feature = "metal"))]
type Surface = (<back::Backend as gfx_hal::Backend>::Surface, back::Instance);

#[cfg(feature = "gl")]
type Surface = (back::Surface);

pub struct Graphics {
    window: (winit::Window, winit::EventsLoop),
    surface: Surface,
    surface_format: format::Format,
    gpu: Gpu<back::Backend>,
}

#[cfg(not(any(feature = "gl", feature = "vulkan", feature = "dx12", feature = "metal")))]
impl Graphics {
    pub(crate) fn new(window_builder: winit::WindowBuilder) -> Result<Graphics, GraphicsInitError> {
        panic!("You must enable one of the backend features!");
    }
}

#[cfg(any(feature = "gl", feature = "vulkan", feature = "dx12", feature = "metal"))]
impl Graphics {

    pub(crate) fn new(window_builder: winit::WindowBuilder) -> Result<Graphics, GraphicsInitError> {
        
        fn acquire_gpu_and_surface(window: &winit::Window) -> Result<(Gpu<back::Backend>, Surface, format::Format), GraphicsInitError> {
            #[cfg(any(feature = "vulkan", feature = "dx12", feature = "metal"))]
            let (mut adapters, mut surface) = {
                let instance = back::Instance::create("mvp engine", 1);
                let surface = instance.create_surface(window);
                let adapters = instance.enumerate_adapters();
                (adapters, (surface, instance))
            };

            #[cfg(feature = "gl")]
            let (mut adapters, mut surface) = {
                let surface = back::Surface::from_window(window);
                let adapters = surface.enumerate_adapters();
                (adapters, (surface))
            };

            let adapter = adapters.remove(0);
            debug!("Using adapter:");
            debug!("{:?}", adapter.info);
            
            let surface_format = {
                let _surface_format = get_surface_format(&adapter, &surface); 
                match _surface_format {
                    Some(f) => f,
                    None => return Err(GraphicsInitError::NoCompatibleSurfaceFormat),
                }
            };

            debug!("{:?}", surface_format);

            Ok((adapter.open_with(|ref family| {
                if family.supports_graphics() && surface.0.supports_queue_family(family) {
                    Some(1)
                } else { None }
            }), surface, surface_format))
        }

        fn get_surface_format(adapter: &Adapter<back::Backend>, surface: &Surface) -> Option<format::Format> {
            surface.0
                .capabilities_and_formats(&adapter.physical_device)
                .1
                .into_iter()
                .find(|format| format.1 == format::ChannelType::Srgb)
        }
        
        info!("Initializing graphics subsystem...");
        let mut events = winit::EventsLoop::new();
        #[cfg(any(feature = "vulkan", feature = "dx12", feature = "metal"))]
        let window = window_builder.build(&events)?;
        #[cfg(feature = "gl")]
        let window = {
            let builder = back::config_context(
                back::glutin::ContextBuilder::new(),
                ColorFormat::SELF,
                None,
            ).with_vsync(true);
            back::glutin::GlWindow::new(window_builder, builder, &events)?
        };
        let window_size = window.get_inner_size_pixels().unwrap();
        info!("Acquiring GPU...");
        let (gpu, surface, surface_format) = acquire_gpu_and_surface(&window)?;

        Ok(Graphics {
            window: (window, events),
            surface: surface,
            surface_format: surface_format,
            gpu: gpu,
        })
    }
}

#[derive(Debug)]
pub enum GraphicsInitError {
    WindowCreationFailed(winit::CreationError),
    NoCompatibleSurfaceFormat,
}

variant_derive_from!(GraphicsInitError:WindowCreationFailed(winit::CreationError));

  