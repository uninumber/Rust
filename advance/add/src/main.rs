use std::ops::Add;
use std::time::Duration;

#[derive(Debug, PartialEq, Clone)]
struct Point<T> {
    x: T,
    y: T,
}

impl <T: Add<Output = T>>Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
           x: self.x + other.x,
           y: self.y + other.y
        }
    }
}

fn main() {

    fn really_add<T: Add<Output = T>>(x: T, y: T) ->  T {
        x + y
    }

    let some = Point {x: 5.0, y: 7.4};
    let floats = some.add(Point {x: 7.2, y: 5.3});

    let some = 5;
    let ints = some.add(4);

    println!("{:?}", floats);
    println!("{}", ints);
    println!("{}", really_add(1.1, 2.44));

    let time = really_add(Duration::new(5, 0), Duration::new(10, 0));
    println!("{:?}", time);
}
