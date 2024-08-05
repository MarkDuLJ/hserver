use std::{sync::{mpsc, Arc, Mutex}, thread};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// create new ThreadPool, if size < 1, panic
    pub fn new(size:usize) -> ThreadPool{
        assert!(size>0);

        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();

        // Error:move occurs because `receiver` has type `std::sync::mpsc::Receiver<Job>`
        let receiver = Arc::new(Mutex::new(receiver));

        // create threads and store in vector
        for id in 0..size {
            workers.push(Worker::new(id,Arc::clone(&receiver)));
        }
        ThreadPool {workers, sender:Some(sender)}
    }


    pub fn excute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();

    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
    }

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id:usize, receiver:Arc<Mutex<mpsc::Receiver<Job>>> ) -> Worker {
        let thread = thread::spawn(move || loop{
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    println!("Woker {id} received a job");
                    job();
                }
                Err(_) => {
                    println!("Woker {id} not a available");
                    break;
                }
            }
        });

        Worker {id, thread:Some(thread)}
    }
}
