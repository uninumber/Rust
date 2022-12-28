fn main() {
    let array1 = [1, 3, 4];
    let array2 = [2, 5, 6];

    let mut iter = array1.iter().zip(array2.iter());

    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
}
