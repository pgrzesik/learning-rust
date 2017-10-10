use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("{} from spawned thread", i);
        }
    });

    for i in 1..5 {
        println!("{} from main thread", i);
    }

    handle.join();
}
