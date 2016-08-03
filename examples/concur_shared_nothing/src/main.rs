use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6];

    let (tx, rx) = channel();

    for elem in &data {
        let (work, tx) = (*elem, tx.clone());
        println!("Send {} to thread", work);
        thread::spawn(move || {
            tx.send(work * work).unwrap();
        });
    }

    for _ in 0..data.len() {
        println!("Main thread received: {}", rx.recv().unwrap());
    }
}
