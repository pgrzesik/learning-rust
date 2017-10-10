use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let cloned_tx = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("Hello there"),
            String::from("Hello there 2"),
            String::from("Hello there 3"),
            String::from("Hello there 4")
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("Hello there from second_thread"),
            String::from("Hello there from second_thread 2"),
            String::from("Hello there from second_thread 3"),
            String::from("Hello there from second_thread 4")
        ];
        for val in vals {
            cloned_tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
