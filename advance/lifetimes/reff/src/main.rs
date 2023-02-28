fn main() {
    let some = {
        let a = 5;
        a
            // &a - error, a drops right after scope. 
            // lifetime is unknown
    };
    println!("{}", some);
}
