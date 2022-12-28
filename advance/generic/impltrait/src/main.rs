
trait Trait {}

impl dyn Trait {
    fn something() -> impl Fn(i32) -> i32 {
        |x| x+ 1
    }
}

fn main() {

    let s: Vec<i32> = vec![1, 2, 3];

    let b = s.iter().map(|n| n+1);
    println!("{:?}", b);

}
