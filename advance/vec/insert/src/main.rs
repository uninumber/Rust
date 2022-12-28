fn main() {
    let mut vec = vec![1, 2, 3];

    vec.insert(3, 7);
    println!("{:?}", vec);
    vec.insert(1, 23);
    println!("{:?}", vec);
    vec.insert(5, 14);
    println!("{:?}", vec);
}

/*
 * pub fn insert(&mut self, index: usize, element: T)
 * Panic if index > len()  
*/

