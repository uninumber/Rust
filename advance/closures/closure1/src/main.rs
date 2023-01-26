fn main() {

    let mut c = vec![21];

    let _c = || {

        let _ = c[0];
        c[0] += 1;
        println!("{:?}", c[0]);
    };

    c[0] += 1;
        println!("{:?}", c[0]);
        println!("{:?}", c);
}
