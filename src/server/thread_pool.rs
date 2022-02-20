//! Module for the thread pool of the http server.

use crate::{
    server::worker::{Message, Worker},
    settings,
};
use log::{debug, info, warn};
use std::{sync::mpsc, sync::Arc, sync::Mutex};

const NUMBER_OF_WORKERS: usize = 10;

pub(crate) struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}
impl ThreadPool {
    pub(crate) fn new() -> ThreadPool {
        let size = match settings::get_number("server.number_of_workers") {
            Ok(number_of_workers) => number_of_workers as usize,
            Err(e) => {
                warn!("{}", e);
                warn!(
                    "The number of workers is not defined. {} taken as default.",
                    NUMBER_OF_WORKERS
                );
                NUMBER_OF_WORKERS
            }
        };
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }
    pub(crate) fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        info!("Sending terminate message to all workers.");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        info!("Shutting down all workers.");
        for worker in &mut self.workers {
            debug!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
