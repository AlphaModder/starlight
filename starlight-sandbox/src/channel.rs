use std::cmp::max;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct StageHandle(usize);

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct ResourceId<T>(usize);

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct ResourceHandle<T>(usize, usize);

pub struct Resource {
    creator: StageHandle,
    buffer_size: usize,
}

pub struct FramePump {
    resources: Vec<Resource>
}

impl FramePump {
    pub fn add_stage<S: BuildStage>(&mut self, build: S) -> S::Output {
        let builder = StageBuilder { 
            handle: 
        }
    }
}

pub struct ResourceBuilder<'a> {
    handle: StageHandle,
    resources: &'a mut Vec<Resource>,
}

impl<'a> ResourceBuilder<'a> {
    pub fn create_resource<T>(&mut self) -> ResourceId<T> {
        self.resources.push(Resource {
            creator: self.handle,
            buffer_size: 1,
        });
    }
}

pub struct StageBuilder<'a> {
    handle: StageHandle,
    resources: &'a mut [Resource]
}

impl<'a> StageBuilder<'a> {
    pub fn read_resource(&mut self, ResourceId<T>, frames_ago: usize) -> ReadHandle<T> {
        
    }

    pub fn write_resource(&mut self, ResourceId<T>) -> WriteHandle<T> {
        
    }
}

pub trait BuildStage {
    type Resources;
    type Stage: FrameStage;
    fn build_resources<'a>(&mut self, &mut ResourceBuilder<'a>) -> Self::Resources;
    fn build_stage<'a>(&mut self, resources: &Self::Resources, &mut StageBuilder<'a>) -> Self::Stage;
}

pub struct Resources {
    fn get_resource<T>
}

trait FrameStage {
    type 
    
}

pub struct FramePump {
    internal_queue: WorkStealingQueue,
}

impl FramePump {
    fn new(parallel_frames: usize) -> (FramePump, Vec<FrameRef>) {

    }
}

impl TaskSource for FramePump {
    fn pop_task(&self) -> Pop<Task> {

    }
}

pub struct FrameTask<'a> {
    pump: &'a FramePump
}

impl FrameTask {
    fn run(&mut self) {
        let state = pump.acquire_stage::<Simulate>().simulate();
    }
}