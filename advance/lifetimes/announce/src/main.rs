use std::fmt::Display;

fn longest<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T
    ) -> &'a str 
where T: Display 
{ 
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {

    let something = "the Rust programming language is interestnig.";
    let b = "the Rust programming language is 33% of Android developement.";
    let result = longest(b, something, "Rust");

    println!("The longest is : {result}");
}

