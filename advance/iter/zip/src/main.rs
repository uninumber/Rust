fn main() {
    let array1 = [1, 3, 4];
    let array2 = [2, 5, 6];


    let mut iter = array1.iter().zip(array2.iter());

    // let all2 = array1.iter().zip(array2);
    // println!("{:?}", all2);

    let some = "Rust is";
    let thing = " amazing programming language";

    let all = [some, thing].join("");
    println!("{}", all);

    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
}
