#[macro_use] extern crate log;
extern crate gfx_hal;
extern crate winit;
extern crate petgraph;
extern crate typemap;
#[macro_use] extern crate futures;
extern crate num_cpus;
extern crate deque;
extern crate rand;
extern crate thread_local;

mod util;
pub mod job;
pub mod graphics;

use job::*;

fn run() {
    let executor = WorkStealingPoolBuilder::new().name_prefix("starlight-worker-").create();
    
}