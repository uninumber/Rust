fn main() {
    let mut b = String::from("Rust is fansy programming language to code in, let's try it.");
    let c = b.find("to").unwrap_or(b.len());

    let some: String = b.drain(..c).collect();
    println!("{}", some);

    let mut num = (0..10).filter(|x| x % 2 == 0).cycle();
    println!("{:?}", num.next());
    println!("{:?}", num.next());
    println!("{:?}", num.next());
    println!("{:?}", num.next());
}
