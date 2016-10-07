use std::thread;
use std::sync::mpsc::channel;

pub fn start() {
    // Create a shared channel that can be sent along from many threads
    // where tx is the sending half (tx for transmission), and rx is the receiving
    // half (rx for receiving).
    let (tx, rx) = channel();
    for i in 0..10 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(i).unwrap();    // кидает в очередь.
        });
    }

    for _ in 0..10 {
        let j = rx.recv().unwrap(); // десять раз читает из очереди
        assert!(0 <= j && j < 10);
        println!("{}", j);
    }
}