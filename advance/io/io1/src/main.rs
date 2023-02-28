fn main() {

    use std::io;

    let mut b = String::new();

    match io::stdin().read_line(&mut b) {
        Ok(n) => {
            println!("{n} bytes presented.");
            println!("{b}");
        },
        Err(err) => println!("Got the error: {}", err)
    }
    //None for Option my man, Err for match, that's it.

    let c = io::stdin().lines();
    for line in c {
        println!("Got a line: {}", line.unwrap());
    }
    //Simple infinite loop with io
}



