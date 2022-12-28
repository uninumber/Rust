fn five_times<T>(x: T)
    where T: Fn(i32) {
        for i in 1..=5 {
            x(i);
        }
    }

fn watsup<T>(mut x: T)
    where T: FnMut() {
        x();
    }

fn main() {

    five_times(|s| -> () {println!("warrup: {}", s)});
    five_times(|something| println!("warrup: {}", something));

    let mut x: i32 = 5;

    {
        let dod  = || x += 1;
        watsup(dod);
    }

    println!("{}", x);
}

