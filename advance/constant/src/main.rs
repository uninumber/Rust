const SOMETHING: &str = "Rust";

const SOMETHING_ELSE: i32 = 5;

fn is_big(x: i32) -> bool {
    x > SOMETHING_ELSE 
}


fn main() {

    let n = 15;

    println!("this is first constant: {}", SOMETHING);
    println!("this is n: {}", n);
    println!("the biggest one is: {}", if is_big(n) {n} else {SOMETHING_ELSE});
}
