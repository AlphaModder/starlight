use std::sync::Arc;

struct Frame {
    id: usize,

}

pub enum FrameStage {
    
}

enum Build<T, B: Builder<T>> {
    Unbuilt(B),
    Built(T),
}

impl<T, B: Builder<T>> Build<T, B> {
    fn new(builder: B) -> Self {
        Build::Unbuilt(builder)
    }
}

trait Builder<T> {
    type Error;
    fn build(self) -> Result<T, Self::Error>;
}


/*
struct ExternalState;

struct GameState(usize);
struct VisibilityData;
struct RenderData;
struct RenderHistory;
struct PresentData;

struct Simulate(GameState),
struct ComputeVisibility(GameState),
struct Extract(VisibilityData, GameState),
struct Present(PresentData),
struct Render(RenderData, RenderHistory);
*/