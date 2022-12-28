fn main() {

    let mut x = vec![1, 2, 3, 3, 3, 4, 5, 5, 5];

    x.dedup();

    println!("{:?}", x);

    /* so removes continuous duplications.. Did you heard that? */
}
