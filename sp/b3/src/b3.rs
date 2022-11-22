struct CustomSmartPoint {
    data: String,
}

impl Drop for CustomSmartPoint {

    fn drop(&mut self) {
        println!("CustomSmartPoint drop that > {}", self.data);
    }
}
fn main() {

    let x = CustomSmartPoint {
        data: String::from("one stuff"),
    };
    println!("CustomSmartPoint created here and next will be dropped...");
    drop(x);
    println!("Dropping CustomSmartPoint..");
}
