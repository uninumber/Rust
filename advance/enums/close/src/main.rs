fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
fn main() {
    let mut something: String = String::from("foo");
    match something {
        _ => for i in 1..=5 {
            something += "bar";
            println!("{something:?}");
        }
    }
    println!("{something:?}");
    print_type_of(&something)
}
