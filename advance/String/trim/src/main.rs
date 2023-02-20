fn main() {
    let b = String::from("something here did you know that?\t\n");
    let s = " English ";
    let c = "  עברי  ";

    println!("{:?}", b.trim_end());
    println!("{:?}", s.trim_end().chars().rev().next());
    println!("{:?}", c.trim_end().chars().rev().next());
    loop {
        println!("hello world");
    }
}
