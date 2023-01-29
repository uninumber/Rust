//There is no Copy for String or &str
#[derive(Debug, Clone)]
struct Stuff{ x: String, b: String}

// struct Stuff (i32);
//
// impl Stuff {
//     fn add(&self) -> i32 {
//         let b = 21;
//         self.0 + b 
//     }
// }

// struct Stuff (String);
impl Stuff {
    fn add(&self) -> String {
        let b = String::from("Rust is perfect programming language.");
        self.x.clone() + &b
    }
}

fn main() {
    let some = Stuff {x: String::from("Hey, "), b: String::from("Bye, ")};
    // let some = Stuff(String::from("Hey, "));

    println!("{:?}", Stuff::add(&some));

    // let some = Stuff(15);
    // println!("{}", some.add());
}
