fn main() {

    let b = vec!["machine learning rust language python hacking"];

    let c = b.iter();
    // we can't iterate through &str or String
    // instead we should use method chars() for &str && String.
    
    println!("{:?}", c);
}
