fn main() {

    
    let mut b = vec!["i", "really", "enjoy", "x", "language", "rust", "what"];
    b.swap_remove(3);
    println!("{:?}", b);
}

/* fn swap_remove(&mut self, index: usize) -> T
 * removes an element and returns it.
 * the element in replaced by the last element of vector?(wow, (tradeoff))
 * the O(1) speed */

