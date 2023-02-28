use std::sync::Arc;

fn call(arg: &Arc<u32>) -> impl Fn(String) -> u8 {
    let cloned = arg.clone();
    move |s| {
        println!("Name: {}", s);
        *cloned as u8
    }
}

fn button(name: &str, f: impl Fn(String) -> u8) {
    println!("{}", f(name.into()))
}
fn main() {
    let arc = Arc::new(3 as u32);
    button("First", call(&arc));
    button("Second", call(&arc));
}
