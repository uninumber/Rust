#[derive(Debug)]
enum Processor {
    Intel (String, u32),
    Ryzen {serial_number: u32, cost: u32},
}

fn main() {

    let b = Processor::Intel("i9-12900k".to_string(), 100);

    let s = Processor::Ryzen{serial_number: 238382, cost: 399};

    println!("{:?} - {:?}", b, s)
}

//Interesting thing - variable your provide in Enum you should use when declaring variables
