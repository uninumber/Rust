
#[derive(Debug)]

struct Rectangle 
{
    width:u32, 
    height:u32,
}

fn main()
{
    let rect1 = Rectangle
    {
        width:30, 
        height:40,
    };

    println!("The area of rect1 is: {:?}.", rect1);
}
