use std::fmt::Debug;

#[derive(Debug)]

struct Stuff<'a, T: 'a>(&'a T);

fn printd<T>(t: T) 
    where T: Debug {
        println!("t is {:?}", t); 
}

fn printd_ref<'a, T: 'a>(t: &'a T) 
where T: Debug + 'a {
    println!("t is {:?}", t);
}

fn main() {

    let b = 8;

    let some = Stuff(&b);
    printd_ref(&some);
    println!("{:?}", some);
}
