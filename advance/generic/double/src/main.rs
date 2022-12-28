fn double<const T: i32> ()  {
    println!("doubled: {}", T * 2)
}

fn main() {
    double::<9>();
}

