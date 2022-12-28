fn main() {
    // damn it, retain can be so useful!


    let mut vec = vec![1, 2, 3, 4, 5, 6];

    vec.retain(|num| num % 3 == 0);
    println!("{:?}", vec);

    // Works only with vec! buddy.

}

/* pub fn retain(&mut self, f: F)
 * where F: FnMut(&T) -> bool */
