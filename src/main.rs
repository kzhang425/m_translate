use m_translate::*;
use std::env;
use std::result;
use std::io;
use std::io::Read;
use std::path::Path;
use std::fs::File;

fn main() {
    // Grab command line args
    let args = env::args();
    for arg in args {
        println!("{}", arg);
    }
}

// Convenience function to open a file and dump its contents into a string. Errors should be handled.
fn open_to_buffer(arg: String) -> io::Result<String> {
    let mut file = File::open(arg)?;
    
    let mut buffer = String::new();
    let size = file.read_to_string(&mut buffer)?;
    println!("{} bytes were read from file.", size);

    Ok(buffer)

    
}

#[cfg(test)]
mod io_tests {
    use super::*;

    #[test]
    fn file_read() {
        let s = open_to_buffer("test.txt".to_string()).unwrap();
        assert_eq!("Hello World".to_string(), s);

    }
}