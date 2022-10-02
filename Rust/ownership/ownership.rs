fn main()
{
    let x = String::from("Hello.");

    take_owner(x);

    let s = 2;
    
    makes_copy(s);
}

fn take_owner(some_string: String) 
{
    println!("{}", some_string);
}
fn makes_copy(some_integer: u32)
{
    println!("{}", some_integer);
}
