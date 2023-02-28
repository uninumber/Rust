//until you don't  manipulate variables and don't return them in function
//you can pass lifetimes.
fn show<'a, 'b>(x: &'a i32, b: &'a i32) {
    println!("this is x : {x}, and this is b: {b}")
}

fn main() {

    let (five, six) = (5, 6);

    show(&five, &six)
}
