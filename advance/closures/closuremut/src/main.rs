fn closure(mut f: impl FnMut()) {f()}
fn main() {

    let mut x: i32 = 10;

    closure(|| {
        x += 10;
        println!("got: {}", x);
    });

    println!("value x afte return: {}", x);
    //so here we can see, that value of x was changed because of closure(mut f: FnMut()) {f()}
}
