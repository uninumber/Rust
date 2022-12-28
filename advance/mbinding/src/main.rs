#[derive(Debug)]
enum Something {
    Number {real_number: u32},
}

fn main() {

    let number = Something::Number {real_number: 10};

    match number {

        Something::Number {real_number: i @ 7..=15, } => {
            println!("Here is your number: {:?}", number);
        },
        Something::Number {real_number: 1..=3} => {
            println!("Oops. IDK what's that.");
        },
        _ => println!("Something, went wrong!")
    }
}
