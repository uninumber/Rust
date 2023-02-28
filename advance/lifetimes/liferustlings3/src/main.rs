struct Book<'a, 'b> {
    author: &'a str,
    title: &'b str,
}

fn main() {
    let name = "Jill Smith";
    let title = "Thinking together";
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}

