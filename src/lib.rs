#[macro_use]
extern crate log;
extern crate winit;

#[macro_use]
mod macros {
    macro_rules! variant_derive_from {
        ($enum:ty:$variant:ident($from:ty)) => {
            impl From<$from> for $enum {
                fn from(thing: $from) -> $enum { $enum::$variant(thing) }
            }
        };
        ($enum:ty:$variant:ident($from0:ty, $($from:ty),*)) => {
            impl From<($from0, $($from, )*)> for $enum {
                fn from(thing: ($from0, $($from)*)) -> $enum { 
                    let ($from0, $($from)*) = thing;
                    $enum::$variant($from0, $($from)*)
                }
            }
        };
    }
}

pub mod graphics;

pub struct Engine {
    graphics: graphics::Graphics,
}

impl Engine {
    
    pub fn new(window_builder: winit::WindowBuilder) -> Result<Engine, EngineInitError> {
        info!("MVP Engine init started.");
        use graphics::Graphics;
        let graphics = Graphics::new(window_builder)?;

        Ok(Engine { 
            graphics: graphics,
        })
    }

    pub fn run(&self) {

    }

}

#[derive(Debug)]
pub enum EngineInitError {
    Graphics(graphics::GraphicsInitError)
}

// variant_derive_from!(EngineInitError:Graphics(graphics::GraphicsInitError));

impl From < graphics::GraphicsInitError > for EngineInitError {
           fn from ( thing : graphics::GraphicsInitError ) -> EngineInitError {
           return EngineInitError :: Graphics ( thing ) } }