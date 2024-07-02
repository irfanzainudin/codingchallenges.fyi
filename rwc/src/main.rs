use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    println!("Welcome to rwc!");

    let args: Vec<String> = env::args().collect();
    let filename = args.last().unwrap();
    for arg in &args {
        match arg.as_str() {
            "-s" => {
                // Returns size of file in bytes
                let mut file = File::open(filename).unwrap();
                print!("{}\n", file.metadata().unwrap().len())
            }
            "-c" => {
                // Returns number of bytes in file
                // TODO: I'm getting 335043 instead of 342190
                let mut file = File::open(filename).unwrap();
                let mut bc = 0;
                for b in file.bytes() {
                    bc += 1;
                }
                print!("{}\n", bc);
            }
            _ => {}
        }
    }
}