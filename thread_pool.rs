use std::thread::{self, JoinHandle};

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

struct ThreadPool {
    workers: Vec<JoinHandle<()>>,
}

type Job = FnOnce() -> ();

impl ThreadPool {
    fn new(worker_count: usize) -> ThreadPool {
        ThreadPool {
            workers: Vec::with_capacity(worker_count),
        }
    }

    fn exec(self, job: Box<Job>) {
        job.call_box();
    }
}

fn main() {
    let t1 = thread::spawn(|| println!("{}", "child thread"));
    println!("{}", "main thread");
    t1.join();
}
