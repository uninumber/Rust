fn main() {
    let mut my_reference: Vec<i32> = vec![5];

    {
        let reference: Vec<i32> = vec![5, 10, 15];

        //but if everything in the same scope
        //then it works
        
        my_reference = reference;
        // my_reference = &reference;
        //borrowed value does not live long enough

        // drop(reference); -- error occurs, because 
        // value `reference` used afere move 
    }

    if let reference = my_reference {
        println!("{:?}", reference);
    }
}
