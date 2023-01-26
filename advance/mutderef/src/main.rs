fn main() {
    let list = [1, 2, 4, 2352, 4233, 23, 1123];

    let some = &list[4];

    // basically if you tryint to modify referenced fucntion with no-referenced function
    // in process you should add "*" as dereference.

    println!("{}", add(*some));
}

fn add(some: i32) -> i32 {
    some + 10
}
