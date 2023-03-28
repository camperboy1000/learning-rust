use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct PoolCreationError;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Creates a new ThreadPool.
    ///
    /// The capacity is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(capacity: usize) -> ThreadPool {
        assert!(capacity > 0);

        let mut workers = Vec::with_capacity(capacity);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..capacity {
            // Make threads and add to vector
            workers.push(Worker::new(id, receiver.clone()));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// Builds a new ThreadPool.
    ///
    /// The capacity is the number of threads in the pool.
    ///
    /// # Errors
    ///
    /// The `build` function will return an error if the capacity is not greater than 0.
    pub fn build(capacity: usize) -> Result<ThreadPool, PoolCreationError> {
        match capacity > 0 {
            true => Ok(ThreadPool::new(capacity)),
            false => Err(PoolCreationError),
        }
    }

    /// Allocats a thread from the ThreadPool and executes the closure.
    ///
    /// The closure is the function to be executed.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shuting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl Worker {
    /// Creates a new Worker thread.
    ///
    /// The id is the id of ther worker thread.
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::Builder::new()
            .spawn(move || loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job, executing...");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected, shuting down...");
                        break;
                    }
                };
            })
            .expect("Failed to create thread in Worker!");

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
