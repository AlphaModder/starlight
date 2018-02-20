#[macro_use]
extern crate log;
extern crate gfx_hal;
extern crate winit;
extern crate petgraph;
extern crate typemap;

mod util;
pub mod graphics;

pub struct Engine<G: graphics::backend::Backend> {
    graphics: graphics::Graphics<G>,
}

impl<G: graphics::backend::Backend> Engine<G> {
    
    pub fn new(window_builder: winit::WindowBuilder) -> Result<Engine<G>, EngineInitError<G>> {
        info!("Starlight init started.");
        Ok(Engine { 
            graphics: graphics::Graphics::<G>::new(window_builder)?,
        })
    }

    pub fn run(&self) {

    }

}

#[derive(Debug)]
pub enum EngineInitError<G: graphics::backend::Backend> {
    Graphics(graphics::GraphicsInitError<G>)
}

impl<G: graphics::backend::Backend> From<graphics::GraphicsInitError<G>> for EngineInitError<G> {
    fn from(graphics: graphics::GraphicsInitError<G>) -> EngineInitError<G> {
        EngineInitError::Graphics(graphics)
    }
}
