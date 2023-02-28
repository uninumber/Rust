fn return_closure() {
    let some = |x: i32| println!("{}", x + x);
    some(7)
}

fn main() {
    return_closure()
}
