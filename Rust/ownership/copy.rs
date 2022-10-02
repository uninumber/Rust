fn main()
{
    let s = String::from("Hello, buddy");

    let x = s.clone();

    println!("x = {}\ns = {}", x, s);
}
//you can clone integers just with "=". But if you want to use another,use: .clone();
//Here are some types that implement Copy:
//* All the integer types, such as u32;
//* The boolean type, bool, with values true and false;
//* All floating types, such as f64;
//* The character types, char;
//* Tuples if they only contain elements such as above.
//* Tuples (i32, i32)/(ok); Tuples (i32, String)/(nope);
