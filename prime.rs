fn main() {

    let p = 21;

    println!("{:?}", is_prime(p));

}

/* I have realized how return actually works in Rust...
 * you can just return - true in the same sense as else return true;*/

fn is_prime(x: u32) -> bool {

    if x < 1 {return false;}

    for i in 2..x {
        if x%i == 0 {
            return false;
        }
    }
        true
}
