
fn main() 
{

enum Message 
{
    Quit, 
    Write(String),
    Move {x:u32, y:u32},
    ChangeColor (u32, u32, u32),
}


impl Message 
{
    fn call(&self) 
    {
    }
}
let m = Message::Write(String::from("hello"));
m.call();
}
