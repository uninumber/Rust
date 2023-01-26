trait Animal {
    fn eat(&self);
}

struct  Herbivore;
struct  Carnivore;

impl Animal for Herbivore {
    fn eat(&self) {
        println!("I eat plants.");
    }
}

impl Animal for Carnivore {
    fn eat(&self) {
        println!("I eat meat.");
    }
}

fn main() {

    let mut something: Vec<Box<dyn Animal>> = Vec::new();
    let h = Herbivore;
    let c = Carnivore;

    something.push(Box::new(h));
    something.push(Box::new(c));

    for animal in something {
        animal.eat();
    }
}
    
