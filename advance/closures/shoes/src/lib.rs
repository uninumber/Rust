#[derive(Debug, Clone, PartialEq)]
struct RAM<'a> {
    brand: &'a str,
    size: i32,
}

fn in_size<'a>(ram: Vec<RAM<'a>>, brand: &'a str) ->  Vec<RAM<'a>> {
    //required by a bound introuduced by this call
    //arise then you try use iter() for Vec<_> instead of into_iter()
    ram.into_iter().filter(|x| x.brand == brand).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn ram_in_size() {
        let my_ram = vec![
            RAM {
                brand: "Kingston",
                size: 2699,
            }
        ];

        let needed_ram = in_size(my_ram, "Kingston");

        assert_eq!(
            needed_ram,
            vec![
                RAM {
                    brand: "Kingston",
                    size: 2699,
                },
            ]
        )
    }
}
