struct Something {u: i32}

use Something as Moro;
type Point = (u8, u8);     

fn main () {

    let _: Point = (10, 20);
    let _ = Moro{u: 5};
}

