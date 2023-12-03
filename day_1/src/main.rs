use std::{fs::File, io::Read};

pub fn part_1(content: &str) {
    let sum = content.split("\n")
        .map(|line| {
            let mut digits = line.chars()
                .into_iter()
                .filter(|val| val.is_ascii_digit())
                .fold(
                    (None, None),
                    |(start, end), next| {
                        
                        match (start, end) {
                            (None, _) => {
                                (Some(next.to_digit(10).unwrap()), None)
                            },
                            (Some(_), None) | (Some(_), Some(_)) => {
                                (start, Some(next.to_digit(10).unwrap()))
                            },
                        }
                });

            match digits{
                (Some(start), None) => {
                    digits = (Some(start), Some(start))
                },
                _ => {}
            };

            digits
        })
        .filter(|(start, end)| start.is_some() && end.is_some())
        .map(|(start, end)| {
            // println!("{start:?}, {end:?}");
            start.unwrap() * 10 + end.unwrap()
        })
        .fold(0, |acc, next| acc + next);

    println!("sum: {sum:?}");
}

fn get_digits(current: &[String]) -> Option<u32> {
    let text: String = current.join("");
    let text = text.trim();

    let digits = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
        .iter()
        .filter(|(num, _)| num.len() <= text.len());

    // println!("\n");
    for (digit, val) in digits {
        // println!("digit:{digit}\tval:{:?}",text.get((text.len() - digit.len())..));
        if *digit == text.get((text.len() - digit.len())..).unwrap() {
            return Some(*val)
        };
    }
    return None
}

fn part_2(content: &str) {
    let sum = content.split("\n")
        .map(|line| {
            let digits = line.chars()
                .into_iter()
                .fold(
                    (vec![], None, None),
                    |(mut char_stack, start, end), next| {
                        match (next.is_ascii_alphabetic(), next.is_numeric()) {
                            (true, false) => {
                                char_stack.push(next.to_string());
                                if char_stack.len() > 5 {
                                    char_stack.remove(0);
                                }
                                // println!("{char_stack:?}");
                                let digit = match char_stack.len() {
                                    3 | 4 | 5 => get_digits(&char_stack),
                                    _ => None
                                };

                                match digit {
                                    Some(val) => {
                                        // char_stack.clear();

                                        match (start, end) {
                                            (None, _) => {
                                                (char_stack, Some(val), None)
                                            },
                                            (Some(_), None) | (Some(_), Some(_)) => {
                                                (char_stack, start, Some(val))
                                            },
                                        }
                                    },
                                    None => (char_stack, start, end)
                                }
                            },
                            (false, true) => {
                                char_stack.clear();

                                match (start, end) {
                                    (None, _) => {
                                        (char_stack, Some(next.to_digit(10).unwrap()), None)
                                    },
                                    (Some(_), None) | (Some(_), Some(_)) => {
                                        (char_stack, start, Some(next.to_digit(10).unwrap()))
                                    },
                                }
                            },
                            (_, _) => {
                                char_stack.clear();
                                (char_stack, start, end)
                            }
                        }
                });

            let mut digits = (digits.1, digits.2);

            match digits{
                (Some(start), None) => {
                    digits = (Some(start), Some(start))
                },
                _ => {}
            };

            // println!("{digits:?}");

            digits
        })
        .filter(|(start, end)| start.is_some() && end.is_some())
        .map(|(start, end)| {
            // println!("{start:?}, {end:?}");
            start.unwrap() * 10 + end.unwrap()
        })
        // .skip(10)
        // .for_each(|val| println!("{val}"));
        .fold(0, |acc, next| acc + next);

    println!("sum: {sum:?}");
}

fn main() {
    let mut f: File = File::open("input.txt").unwrap();

    let mut content = String::new();

    let _result = f.read_to_string(&mut content);

    part_1(&content);

    part_2(&content);
}