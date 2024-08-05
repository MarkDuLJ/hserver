use std::{sync::{mpsc, Arc, Mutex}, thread};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
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
        ThreadPool {workers, sender}
    }


    pub fn excute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();

    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id:usize, receiver:Arc<Mutex<mpsc::Receiver<Job>>> ) -> Worker {
        let thread = thread::spawn(move || loop{
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Woker {id} received a job");
            job();
        });

        Worker {id, thread}
    }
}
