use std::{vec::Vec, thread};

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    Launch(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: crossbeam_channel::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "number of threads must be positive integer.");
        
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = crossbeam_channel::unbounded::<Message>();

        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()))
        }
        
        ThreadPool {
            workers,
            sender,
        }
    }

    pub fn launch<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Message::Launch(Box::new(f))).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("dropping thread pool");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        
        for worker in &mut self.workers {
            println!("trying to join thread {}", worker.id);
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
    fn new(id: usize, receiver: crossbeam_channel::Receiver<Message>) -> Worker {
        let thread = Some(thread::spawn(move || {
            println!("{}: thread starting", id);
            loop {
                match receiver.recv().unwrap() {
                    Message::Launch(job) => {
                        println!("{}: thread lauching job", id);
                        job();
                        println!("{}: thread finishing job", id);
                    },
                    Message::Terminate => break,
                }
            }
            println!("{}: thread stopping", id);
        }));

        Worker { id, thread }
    }
}