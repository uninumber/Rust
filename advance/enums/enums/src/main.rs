#[derive(Debug)]
enum Something { /* just c-like enum */
    FirstVariant,
    SecondVariant, 
    ThirdVariant,
}

#[derive(Debug)]
enum Something2 { /* this is enum with 2 anonymous and 1 known value */
    Uknown,
    Anonymous,
    Known(u8),
}

#[derive(Debug)]
enum Something3 { /* this is enum which consists of struct */

    Struct {
        ip: u32,
        tcp: u32,
    }
}

enum EmptyEnum {}

fn main() {

    let b = Something3::Struct{ip: 14, tcp: 15};
    println!("{:?}", b);


    let s = Something2::Known(6);
    println!("{:?}", s);


    let x = Something::FirstVariant;
    println!("{:?}", x);

    //let m = EmptyEnum; the emtpy enums (!) can not be instatiated at all.
}

