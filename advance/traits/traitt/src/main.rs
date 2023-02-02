//T contains all owned types. T is a superset of both 
//&T and &mut

//&T and &mut T are disjoint sets
trait Stuff {}

// impl<T> Stuff for T {}

impl<T> Stuff for &T {}

impl<T> Stuff for &mut T {}

fn main() {}
