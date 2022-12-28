fn main() {

    let mut x = vec![1, 2, 3, 4, 5];

    x.truncate(3);

    println!("{:?}", x);
}
