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
    fn new(&self) -> &T;
}

impl<'a, T> Something for Stuff<'a, T> {
    fn new(&self) -> &T {
         &self.x 
    }
}

fn main()  {

    let some = Stuff {x: &5};
    println!("{:?}", some.new());

}
