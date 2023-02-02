#[derive(Debug)]
struct Gentoo;
truct Void;

trait VoidDistro {
    fn sound(&self) -> String;
    fn updating(&self) -> String;
}

trait Distro {
    fn sound(&self) -> String;
}

impl Distro for Gentoo {
    fn sound(&self) -> String {
        "brrrrr".to_string()
    }
}

impl VoidDistro for Void {
    fn sound(&self) -> String {
        "shhhh..".to_string()
    }

    fn updating(&self) -> String {
        "updating the packages...".to_string()
    }
}

fn main() {

 //   let gentoo_user = Gentoo;
    let void_user = Void;

  //  println!("{}", gentoo_user.sound());
    println!("{}", void_user.sound());
    println!("{}", void_user.updating());

    /*
      - i think this is generally better example of "how to use traits and structs", than something
      - before with cmp(lol:{}:?).
   */
}

