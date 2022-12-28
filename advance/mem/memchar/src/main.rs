fn main() {

    let x = [8];


    use ::std::mem;
    let size: usize = x.into_iter().map(|m| mem::size_of_val(&m)).sum();
    println!("{:?}", size);

    /* so i was interested why do we use usize specifically? The answer is:
     * pub fn size_of_val<T>(val: &T) -> usize
     * where 
     * T: ?Sized,
     * so, the size can be unknown 
     * 1 character 4 bytes, respectively. 
     * Oh, wow. The size of one digit is also 4 bytes. Actually I supposed that during reading of
     * UTF-8 wiki. everything represented as 2 4-bytes
     * but the function is very interesting. 
     * fn size_of_val<T>(val: &T) -> usize */


    println!("{:?}", mem::size_of::<char>());

}

