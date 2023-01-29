fn main() {

    let b: Option<i32> = Some(5);

    if let Some(5) = b {
        println!("yeap");
    } else {
        println!("nope");
    }
}

