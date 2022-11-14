fn main() {

    use std::collections::HashMap;

    let mut x = HashMap::new();

    x.insert(String::from("Linus"), 9);
    x.insert(String::from("Ritchie"), 3);

    for (d, b) in &x  {

        println!("{}", b);
    }
}
