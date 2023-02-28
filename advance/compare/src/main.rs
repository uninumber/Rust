use std::cmp::Ord;

fn largest<T: Ord>(collection: &[T]) -> &T {
    let mut largest = &collection[0];

    for item in collection {
        if item > largest {
            largest = item;
        }
    }
    largest 
}

fn main() {

    let b = [1, 3, 5, 6, 7, 10];
    let result = largest(&b);
    println!("{:?}", result);
    let b = ['a', 'b', 'c', 'd'];
    let result = largest(&b);
    println!("{:?}", result);
}
