fn bigger<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn main() { 
    let x = "something";
    let r = "Rust is amazing programming language";

    println!("The biggest is : {}", bigger(r, x));
}

//Ok, I found that if we have only one element, Rust understand 
//what is up here and there. But if we add more elements. 
//Shit happens...
