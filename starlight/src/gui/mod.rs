use cgmath;

mod style;
mod backend;

use self::backend::*;

pub use self::style::*;

type Vertex = cgmath::Vector2<f32>;
type CharTransform = cgmath::Matrix3<f32>;

pub struct GuiContext<B: GuiBackend> {
    backend: B,
}

impl<B: GuiBackend> GuiContext<B> {
    pub fn new(backend: B) -> GuiContext<B> {
        Gui { backend: backend }
    }

    pub fn begin(&mut self) -> Gui<B> {
        Gui { context: self }
    }
}

pub struct Gui<'a, B: GuiBackend> {
    context: GuiContext<'a, B>,
    recorder: B::CommandRecorder,
}

impl<'a, B: GuiBackend> Gui<'a, B> {
    pub fn draw(&mut self, verticies: &[Vertex], style: &StyleMap) {

    }
    
    pub fn draw_indexed(&mut self, verticies: &[Vertex], indicies: &[usize], style: &StyleMap) {

    }

    pub fn draw_text(&mut self, text: &str, transforms: &[CharTransform], style: &StyleMap) {

    }

    pub fn finish(self) -> B::Commands {
        self.recorder.finish()
    }
}

