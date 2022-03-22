// On-Stack Dynamic Dispatch
// https://rust-unofficial.github.io/patterns/idioms/on-stack-dyn-dispatch.html

use std::io;
use std::fs;
fn main() {
    let arg = "test";

    // These must live linger than 
    let (mut stdin_read, mut file_read);

    let _readable: &mut dyn io::Read = if arg == "-"  {
        stdin_read = io::stdin();
        &mut stdin_read
    } else {
        file_read = fs::File::open(arg).unwrap();
        &mut file_read
    };

    // use readable here

    println!("Hello, world!");
}
