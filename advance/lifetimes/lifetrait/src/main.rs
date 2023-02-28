// #[derive(Debug)]
// struct Stuff<'a> {
//     x: &'a i32
// }
//
// trait Something {
//     fn new() -> &'static str;
// }
//
// impl<'a> Something for Stuff<'a> {
//     fn new() -> &'static str {
//         "something"
//     }
// }

#[derive(Debug)]
struct Stuff<'a, T: 'a> {
    x: &'a T
}

trait Something { 
    fn new(&self);
}

impl<'a, T: std::fmt::Display> Something for Stuff<'a, T> {
    fn new(&self) {
         println!("The value is : {0}", self.x)
    }
}

fn main()  {

    let some = Stuff {x: &6};
    println!("{:?}", some.new());

}
