use std::mem;

/*
#[derive(Debug)]
struct Something {
    x: i32
}

#[derive(Debug)]
struct Anothersomething {
    z:i32
}
*/

fn main() {

    //let mut x = Something{x: 2};
    //let mut b = Anothersomething{z: 4};

    let mut x = 5;
    let mut b = 10;

    mem::swap(&mut x, &mut b);
    println!("{}-{}", x, b);
    //Ok here was interested is it even possible to swap 'char' with digit, or &str with digit.
    //No.
    //But that absolutely possible swap 'char' with 'char' and &str and String with String
    //Ok, but you can't to do similar things with enums :(
    //And with struct too. :(
    //
    //Do not forget to use &mut for values. You are changing them in the end!
    //
    //pub fn swap<T>(x: &mut T, y: &mut T) - it returns nothing just swap items 
}
