trait ComputerParts {
    fn stream(&self) {}
}

  /* 
      * This is basically structs 
  */

struct Cpu;

struct Gpu;

impl ComputerParts for Cpu {
    fn stream(&self) {
        println!("We have 14599 bytes per second.");
    }
}

impl ComputerParts for Gpu {
    fn stream(&self) {
        println!("We have 25599 bytes per second.");
    }
}

fn main() {
    let mut l: Vec<Box<dyn ComputerParts>> = Vec::new();
    l.push(Box::new(Cpu));
    l.push(Box::new(Gpu));
    l.pop();
    for i in l {
        i.stream()
    }
}

