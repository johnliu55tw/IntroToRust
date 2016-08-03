use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::channel;

fn main() {
    // Create a shared_data protected by Mutex
    let shared_data = Arc::new(Mutex::new(Vec::new()));
    // How many thread do you want, sir?
    let thread_num = 10;
    // For synchronization
    let (tx, rx) = channel();
    // Work!
    for i in 0..thread_num {
        let (idx, shared_data, tx) = (i, shared_data.clone(), tx.clone());
        thread::spawn(move || {
            let mut guard = shared_data.lock().unwrap();
            guard.push(idx);
            tx.send(()).unwrap();
        });
    }
    // Wait for threads...
    for _ in 0..thread_num {
        rx.recv().unwrap();
    }
    println!("Done. vec: {:?}", shared_data.clone());
}
