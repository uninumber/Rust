#[derive(Debug, PartialEq)]

struct Foo<T> {
    a: i32,
    b: T 
}

fn main() {
}

// - the generated implementation for PartialEq is equal to: 
// - impl<T: PartialEq> PartialEq for Foo {
// - fn equal(&self, other: &Foo<T>) {
// -self.a == other.a && self.b == other.b
// - }
