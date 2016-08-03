use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread;

fn sum_vec(vector: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for elem in vector {
        sum += *elem;
    }
    sum
}

fn main() {
    // A LAAAARRRRGE DATA!!!
    let mut data = vec![];
    for i in 0..1000 {
        data.push(i);
    }
    println!("Main: sum of the data should be: {}", sum_vec(&data));

    // For threads to send back the result
    let (tx, rx) = channel();
    // The Arc
    let arc = Arc::new(data);
    // How many threads do you want, sir?
    let thread_num = 10;
    // Work!!!
    for _ in 0..thread_num {
        let tx = tx.clone();
        let arc = arc.clone();
        thread::spawn(move || {
            // arc.push(3); <- This will failed
            tx.send(sum_vec(&arc)).unwrap();
        });
    }
    // Gathering data
    for _ in 0..thread_num {
        println!("Main recv: {}", rx.recv().unwrap());
    }
}
