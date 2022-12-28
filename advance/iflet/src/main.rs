fn main() {
    let s = Some(5);
    let b:  Option<i32> = None;
    let a:  Option<i32> = None;

    if let Some(i) = s {
        println!("matched : {}", i);
    }


    if let Some(l) = b {
        println!("Got this: {}", l);
    } else {
        println!("Got anything.");
    }
}

