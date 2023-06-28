use m_translate::*;
use std::env;

fn main() {
    // Grab command line args
    let args = env::args();
    for arg in args {
        println!("{}", arg);
    }
}
