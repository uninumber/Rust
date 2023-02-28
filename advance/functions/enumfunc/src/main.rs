pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        input.into_iter()
            .map(|(mut string, command)| match command {
                Command::Append(x) => {
                    string.reserve("bar".len() * x);
                    string.extend(std::iter::repeat("bar").take(x));
                    string
                },
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string()
                })
        .collect()

        // let mut output: Vec<String> = vec![];
        // for (mut string, command) in input.into_iter() {
        //     match command {
        //         Command::Append(x) => {
        //             for _ in 0..x {
        //                 string += "bar";
        //                 println!("{:?}", output);
        //             }
        //             output.push(string)
        //         } 
        //         Command::Uppercase => output.push(string.to_uppercase()),
        //         Command::Trim => output.push(string.trim().to_string())
        //     }
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::Command;
    use super::my_module::transformer;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
use crate::my_module::*;
fn main() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        println!("{:?}", output);
}
