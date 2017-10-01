use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("File creation failed: {:?}", e)
                }
            }
        },
        Err(error) => {
            panic!("Problem with opening file: {:?}", error)
        }
    };

    let f_unwrap = File::open("hello.txt").unwrap();
    // Example of unwrap usage to shorten code (Ok(res) => res and Err(e) => panic!("...")
}
