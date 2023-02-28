fn main() {
    let some: Vec<String> = ["some", "thing", "here"].iter().map(|&s| s.into()).collect();
    println!("{:?}", some);
}
