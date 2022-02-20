//! Module for the workers of the http server.

use log::debug;
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;
pub(crate) struct Worker {
    pub(crate) id: usize,
    pub(crate) thread: Option<thread::JoinHandle<()>>,
}
pub(crate) enum Message {
    NewJob(Job),
    Terminate,
}
impl Worker {
    pub(crate) fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    debug!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    debug!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
