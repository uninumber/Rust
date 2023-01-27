use std::ops::Add;
use std::ops::Sub;
use std::time::Duration;

#[derive(Debug, PartialEq, Clone)]
struct First<T> {
    x: T,
    y: T,
}

#[derive(Debug, PartialEq, Clone)]
struct Second<T> {
    x: T,
    y: T,
}

impl <T: Sub<Output = T>>Sub for Second<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self  {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

// impl <T: Add<Output = T>>Add for Point<T> {
//     type Output = Self;
//
//     fn add(self, other: Self) -> Self::Output {
//         Self {
//            x: self.x + other.x,
//            y: self.y + other.y
//         }
//     }
// }

fn main() {

    // fn really_add<T: Add<Output = T>>(x: T, y: T) ->  T {
    //     x + y
    // }

    fn really_sub<T: Sub<Output = T>>(x: T, y: T) -> T {
        x - y
    }

    let num = Second {x: 12.4, y: 14.7};
    let floats = num.sub(Second {x: 12.4, y: 14.7});
    println!("{:?}", floats);

    // let some = Point {x: 5.0, y: 7.4};
    // let floats = some.add(Point {x: 7.2, y: 5.3});

    let some = 5;
    let ints = some.add(4);

    // println!("{:?}", floats);
    
    println!("{}", ints);
    println!("{}", really_sub(1.1, 2.44));

    let time = really_sub(Duration::new(15, 0), Duration::new(10, 0));
    println!("{:?}", time);
}
