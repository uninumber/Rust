#[derive(Debug)]
enum Num {
        One, //0
        Two = 255,
        Three,
}

fn main() {

    let b = Num::One as u8;
    println!("{:?}", b);
    
    let c = Num::Two as u8;
    println!("{:?}", c);

    let d = Num::Three as u32;
    /* interesting thing if it specified as u8 then when it accrosses 255 value it start with 0,
     * but if it has higreh up-bound(u16-..) then it counts + 1 */
    println!("{:?}", d);
}

//if enum is not specified somehow, then conv to u/i value it will start from 0
