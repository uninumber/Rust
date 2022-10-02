struct Rectangle 
{
    width:u32,
    height:u32,
}

fn main()
{
    let rect1 = Rectangle
    {
        width:40, 
        height: 70,
    };


    println!("The area of rectangle is {} square pixels.", area(&rect1));
}

fn area(d: &Rectangle)->u32
{
    d.width * d.height
}
