fn main() {
    let mut x: Vec<i32> = Vec::with_capacity(10);

    x.extend([1, 2, 3]);
    assert_eq!(x.capacity(), 10);

    let slice = x.into_boxed_slice();
    assert_eq!(slice.into_vec().capacity(), 3);

}
