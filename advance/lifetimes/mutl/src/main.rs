// struct Owner (i32, i32, i32);

// impl Owner {
// fn add(&mut self) {println!("{}",  self.0 + self.1 + self.2)}
//     println!("{}", self.0);
// }
// }

//oh goodness we don't fucking need lifetimes for (i and u) / there is a magic behing numbers
fn add_new<'a>(x: &'a  str, b: &'a  str) -> String {
    let d = "really cool";
    let c = " important happenend right now";
    if x.len() > b.len() {
        x.to_owned() + c
    } else {
        b.to_owned() + d
    }
}

fn main() {
    let  b = "something";
    let  c = "some";

    // let mut owner = Owner(18, 10, 21);

    println!("{:?}", add_new(b, c));
}
