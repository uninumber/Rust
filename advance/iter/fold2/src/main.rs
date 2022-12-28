fn main() {


    let array = [1, 2, 3, 4, 5];

    let result = array.iter().fold(0, |acc, x| acc + x);

    println!("{}", result);
}

