use std::cmp::PartialOrd;

use std::fmt::Display;

struct Pair<U> {
    x: U,
    y: U,
}

impl<U> Pair<U> {
    fn somethin(x: U, y: U) -> Self {
        Self {x, y}
    }
}

impl<U> Pair<U> where U: Display + PartialOrd {
    fn cmp(&self) {
        if self.x > self.y {
            println!("the largest component is: {}", self.x);
        } else {
            println!("the largesest component is: {}", self.y);
        }
    }
}

fn main() {

    let s = Pair {x: 10, y: 30};

    println!("{:?}", s.cmp());
}
