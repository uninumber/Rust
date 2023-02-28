#[derive(Debug)]
enum Something {

    Boq,
    Bom, 
    Bor(u32),
}

fn main() { 

    let _a = Something::Boq;
    let  b = Something::Bom;

    let c = Something::Bor(5);

    if let Something::Bom  = b {
        println!("got value: {:?}", b);
    } else {
        println!("got anything...");
    }


    if let Something::Bor(_)  = c {
        println!("got: {:?}", c);
    } else {
        println!("got anything.")
    }

}

