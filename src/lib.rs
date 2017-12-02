#[macro_use]
extern crate log;
extern crate winit;

#[macro_use]
mod macros {
    // doesn't support enums with lifetime parameters because you can't do that
    // also for stupid reasons the final :: has to be a :
    macro_rules! variant_derive_from {
        ($($enum_path:ident)::+:$variant:ident($from:ty)) => {
            impl From<$from> for $($enum_path)::+ {
                fn from(thing: $from) -> $($enum_path)::+ { $($enum_path::)+$variant(thing) }
            }
        };
        ($($enum_path:ident)::+<$($param:ident),+>:$variant:ident($from:ty)) => {
            impl From<$from> for $($enum_path::)+<$($param),+> {
                fn from(thing: $from) -> $($enum_path::)+<$($param),+> { $($enum_path::)+<$($param),+>::$variant(thing) }
            }
        };
        ($($enum_path:ident)::+:$variant:ident($($from:ty),+)) => {
            impl From<($($from),+)> for $($enum_path)::+ {
                fn from(thing: ($($from),+)) -> $($enum_path)::+ { 
                    let ($($from),+) = thing;
                    $($enum_path::)+$variant($($from),+)
                }
            }
        };
        ($($enum_path:ident)::+<$($param:ident),+>:$variant:ident($($from:ty),+)) => {
            impl From<($($from, )+)> for $($enum_path::)+<$($param),+> {
                fn from(thing: ($($from),+)) -> $($enum_path::)+<$($param),+> { 
                    let ($($from),+) = thing;
                    $($enum_path::)*<$($param),+>::variant($($from),+)
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

variant_derive_from!(EngineInitError:Graphics(graphics::GraphicsInitError));