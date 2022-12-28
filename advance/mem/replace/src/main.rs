use std::mem;

fn main() {

    let mut x = vec![1, 2, 3, 4];

    let b = mem::replace(&mut x, vec![5, 6, 7, 8]);

    println!("{:?}-{:?}", x, b);

    //mem::replace is quite powerful thing. you declare a new vec which will contain values of
    //previos vec and previoius vec will contain values spciefied in parent as 2-nd arguement.
    //fn replace(dest: &mut self, value: T) -> T
}

