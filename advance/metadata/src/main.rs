use std::fs;
use std::error::Error;
// use std::path::{PathBuf, Path};

pub struct Config {
    pub path: Vec<String>
}

// pub type MyResult<T> = Result<T, Box<dyn Error>>;
fn main() -> std::io::Result<()> {
    for entry in fs::read_dir("something")? {
        let dir = entry?;
        println!("{:?}", dir.path());
    }
    Ok(())
}
