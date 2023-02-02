fn main() {

    let mut b = [1, 21, 3, 5, 6];
    let mut t: Vec<()> = b.iter_mut().map(|x| *x+=1).collect();
    println!("{:?}", t.next());
    println!("{:?}", t.next());
    println!("{:?}", t.next());
    //println!("{:?}", b.iter().map(|n| if n > n.next() {*n} ));

}
