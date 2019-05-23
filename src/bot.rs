use std::sync::mpsc;

pub fn run_bot(tx: &mpsc::Sender<String>, rx: &mpsc::Receiver<String>) {
    // Wait for application key from main thread
    let key = match rx.recv() {
        Ok(st) => st,
        Err(_) => return
    };
}
