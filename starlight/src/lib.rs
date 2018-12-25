#![feature(proc_macro, proc_macro_non_items, generators, pin)]

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
extern crate serde;
extern crate cgmath;
extern crate crossbeam_deque;

mod util;
pub mod job;
pub mod graphics;
pub mod gui;
// pub mod serialization;

use job::*;

fn run() {
    let executor = WorkStealingPoolBuilder::new().name_prefix("starlight-worker-").create();
}