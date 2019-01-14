extern crate rand;

use rand::random;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time;

type Input = String;
type Output = String;

#[derive(Debug)]
pub struct SlowBackgroundProcessor {
    pending: Sender<Input>,
    completed: Receiver<Output>,
    handle: thread::JoinHandle<()>,
}

impl SlowBackgroundProcessor {
    pub fn new() -> SlowBackgroundProcessor {
        let (pending, pending_rx) = channel();
        let (completed_tx, completed) = channel();

        let handle = thread::spawn(move ||
            process_forever(pending_rx, completed_tx));

        SlowBackgroundProcessor {
            pending,
            completed,
            handle,
        }
    }

    pub fn send(&self, input: Input) {
        if let Err(e) = self.pending.send(input) {
            eprintln!("Failed to send input because channel is closed: {}", e);
        }
    }

    pub fn next_processed(&self) -> Option<Output> {
        self.completed.try_recv().ok()
    }

    pub fn close(self) {
        drop(self.pending);
        if let Err(e) = self.handle.join() {
            eprintln!("Failed to wait for slow background process to exit: {:?}", e);
        }

        while let Ok(text) = self.completed.try_recv() {
            eprintln!("Unhandled processed input: {}", text);
        }
    }
}

fn process_forever(input: Receiver<Input>, output: Sender<Output>) {
    while let Ok(input) = input.recv() {
        let sec = random::<u64>() % 10;
        let wait = time::Duration::from_secs(sec + 1);

        println!("Processing input '{}' for {:?}", input, wait);
        thread::sleep(wait);

        if let Err(e) = output.send(format!("Processed input '{}' for {:?}", input, wait)) {
            eprintln!("Failed to send processed result: {}", e);
        }
    }
}