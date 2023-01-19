fn triple<const T: i32> ()  {
    println!("doubled: {}", T * 2)
}

fn main() {
    triple::<9>();
}

//what's up
