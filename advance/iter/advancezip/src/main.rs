//use std::iter::zip;

fn main() {

    let arr = [1, 2, 3];

    let arr2 = [4, 5, 6];

/*    let mut something = zip(
        arr.iter().map(|x| x*2).skip(1),
        arr2.iter().map(|x| x*2).skip(1),
        );
*/

    let mut zip = arr.iter()
        .map(|x| x*2)
        .skip(1)
        .zip(arr2.iter().map(|x| x*2).skip(1));

    println!("{:?}", zip.next());
    println!("{:?}", zip.next());
    println!("{:?}", zip.next())


}
