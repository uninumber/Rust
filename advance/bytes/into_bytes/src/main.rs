use std::str;
fn main() {
    let some = "some";
    let s = &some[..];
    println!("{:?}", s.bytes());
    let b = s.as_bytes();
    let c = str::from_utf8(&b).unwrap();
    println!("{:?}", c);
    println!("{:?}", b"some");

}
