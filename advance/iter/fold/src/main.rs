fn main() {

    let arr = [1, 2, 3, 4];

    let zero = 0.to_string();

    let result = arr.iter().fold(zero, |acc, x| {
        format!("({acc} + {x})")
    });

    println!("{}", result);
}
