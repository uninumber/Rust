use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let mut f = File::open("some.txt")?;

    // let mut reader = BufReader::new(f);
    //
    // let mut line = String::new();
    //
    let mut bytes = [0;32];

    let something = f.read(&mut bytes[..])?;

    println!("The len of line is: {something} in bytes.");

    Ok(())
}
