use std::env;

fn main() {
    let b = vec![String::from("first"), String::from("second")];

    let c = String::from("somethingelsesomethinglsesomethingelse");
    println!("{:?}, {:?}", c.capacity(), c.len());
    println!("{:?}", b.len());
    for argument in env::args_os() {
        println!("{argument:?}");
    }
}
