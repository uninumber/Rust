trait MyArray {
    fn hi(&self) -> [u8;4] {
        self.into()
    }
}

impl MyArray for &str {}

fn main() {
    let a = "something";
    
    println!("{:?}", a.hi());
}
