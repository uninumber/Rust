use std::io::{self, BufRead, BufReader, Write};
use std::fs::{File, self};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // let my_file = File::open("some.txt")?;
    // let buf = BufReader::new(my_file);
    //
    // for some in buf.lines() {
    //     println!("the lines is: {:?}", some);
    // }

    let line = String::new();
    let previous = String::new();
    loop {
        let my_file = fs::read_to_string("some.txt")?;

        if line.trim_end() != previous.trim_end() {

        }
    }

    
    // println!("{:?}", my_file);

    Ok(())
}

