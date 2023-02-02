trait Newspaper {
    fn read() -> String;
}

struct Reader;


impl Reader {
    fn read() -> String {
        String::from("oh, wow, what a newspaper!")
    }
}

impl Newspaper for Reader {
    fn read() -> String {
        String::from("that's for you, read with enjoyness.")
    }
}

fn main() {

    println!("{}", <Reader as Newspaper>::read());
    println!("{}", <Reader>::read());
}
//functions without any given value and type in it called associated functions.
