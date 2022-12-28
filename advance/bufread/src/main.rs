use std::io::prelude::*;

use std::io::BufReader;

use std::fs::File;

fn main() -> std::io::Result<()>  {

    let f = File::open("some.txt")?;

    let mut reader = BufReader::new(f);

    let mut line = String::new();

    let len = reader.read_line(&mut line)?;

    println!("The len of line is: {len} in bytes.");

    Ok(())
}
