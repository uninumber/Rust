fn main() {

    let x: Vec<i32> = Vec::with_capacity(10);
    println!("capacity of x: {:?}", x.capacity());

    let x = vec![1, 2, 3, 4, 5];

    let slice = x.into_boxed_slice();
    println!("capacity of slice: {:?}", slice.into_vec().capacity());

    //To properly change capacity you need to put vec![] right after asking for capacity. 
    // So 1. Vec::with_capacity().
    // println!()
    // vec![]
}

