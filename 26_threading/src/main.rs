use std::thread;

fn main() {
    let v = vec![1, 2, 3, 4];

    let handle = thread::spawn(move || {
        println!("Hello from spawned thread: {:?}", v);
    });

    for i in 1..5 {
        println!("{} from main thread", i);
    }

    handle.join();
}
