#[derive(Debug)]

//This just an example of enum

enum Tech
{
    C, 
    Linux
}

enum Authors
{
    LinusTorvalds(Tech),
    DennisRitchie(Tech)
}

fn creation(author: Authors) -> u32 {
    match author
    {
        Authors::LinusTorvalds(os) => {
            println!("Linus Torvalds has created: {:?}", os);
        1
        }
        Authors::DennisRitchie(language) => {
            println!("Linus Torvalds has created: {:?}", language);
        1
        }
    }
}

fn main() {
    creation(Authors::LinusTorvalds(Tech::Linux));
    creation(Authors::DennisRitchie(Tech::C));
}



