use rand;
use crossbeam_deque::{self as deque, Worker, Pop, Stealer, Steal};
use super::mpsc_queue::{self as mpsc, Producer, Consumer};

fn rand_in_range(range: Range<usize>) -> usize {
    rand::thread_rng().gen_range(range.start, range.end)
}

pub struct Task {

}

pub trait PopExt {
    fn map_none(self, other: Self) -> Self;
}

impl<T> PopExt for Pop<T> {
    fn map_none(self, other: Pop<T>) -> Pop<T> {
        if let Pop::Empty = self {
            other
        }
        else {
            self
        }
    }
}

pub trait StealExt<T> {
    fn into_pop(self) -> Pop<T>;
}

impl<T> StealExt<T> for Steal<T> {
    fn into_pop(self) -> Pop<T> {
        match self {
            Steal::Data(data) => Pop::Data(data),
            Steal::Empty => Pop::Empty,
            Steal::Retry => Pop::Retry,
        }
    }
}

pub trait TaskSource {
    fn pop_task(&mut self) -> Pop<Task>;
}

pub struct LocalQueue {
    external: Consumer<Task>,
    internal: Worker<Task>,
}

impl TaskSource for LocalQueue {
    fn pop_task(&mut self) -> Pop<Task> {
        let result = loop {
            match self.external.pop() {
                Pop::Data(task) => self.internal.push(task),
                r @ _ => break r,
            }
        };
        self.internal.pop().map_none(result)
    }
}

pub struct StealQueues {
    stealers: Vec<Stealer<Task>>,
}

impl TaskSource for StealQueues {
    fn pop_task(&mut self) -> Pop<Task> {
        pool.stealers[rand_in_range(0..self.stealers.len())].steal().into_pop()
    }
}