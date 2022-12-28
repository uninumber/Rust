use std::mem;

fn main() { 

    let mut x = vec![2, 3, 4];

    let b = mem::take(&mut x);

    assert_eq!(vec![2, 3, 4], b);
    // If you want to compare do not use [2, 4]. You compare not an array, but a vec![] in the end.
    assert!(x.is_empty());
    // so in simple words mem::take() takes ownership and makes previos variable, struct empty.
    //
    
    struct Buffer<T> { buf: Vec<T>}

    impl <T> Buffer<T> {
        fn get_and_reset(&mut self) -> Vec<T> {
            mem::take(&mut self.buf)
        }
    }
    
    let mut buffer = Buffer {buf: vec![1, 2, 3]};

    assert_eq!(buffer.buf.len(), 3);
    assert_eq!(buffer.get_and_reset(), vec![1, 2, 3]);
    assert_eq!(buffer.buf.len(), 0);

}

