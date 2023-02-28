use std::iter::zip;

fn main() {

    let arr = [1, 2, 3];

    let arr2 = [4, 5, 6];

    //next() require to be mutable
    let mut something = zip(
        arr.iter().map(|x| x*2).skip(1),
        arr2.iter().map(|x| x*2).skip(1),
        );

    println!("{:?}", something.next());
    println!("{:?}", something.next());
    println!("{:?}", something.next());


    let mut some = arr.iter()
        .map(|x| x*2)
        .skip(1)
        .zip(arr2.iter().map(|x| x*2).skip(1));

    println!("{:?}", some.next());
    println!("{:?}", some.next());
    println!("{:?}", some.next());


    // let a = [1, 2, 3];
    // let b = &a;
    //
    // println!("{}", b.iter().count());
}
