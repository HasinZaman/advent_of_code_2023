use std::{fs::File, io::Read, collections::HashMap, cmp::max};

fn game_id(id: &str) -> u32 {
    let parts = id.trim().split(" ");

    parts.last().unwrap().parse::<u32>().unwrap()
}

fn get_game(raw_str: &str) -> HashMap<&str, u32> {
    let raw_str = raw_str.trim();

    raw_str.split(";")
        .map(|set| {
            set.split(",")
                .map(|pick| {
                    let mut val = pick.trim().split(" ");

                    let num = val.next().unwrap().parse::<u32>().unwrap();
                    let colour = val.next().unwrap();

                    (colour, num)
                })
                .collect::<HashMap<&str, u32>>()
        })
        .fold(
            HashMap::new(),
            |mut acc, val| {
                for (key, val) in val {
                    match acc.contains_key(key) {
                        true => {
                            acc.insert(key, max(acc[key], val));
                        },
                        false => {
                            acc.insert(key, val);
                        }
                    }
                }

                acc
            }
        )
}

fn part_1(content: &str) {
    let sum = content.split("\n")
        .filter(|line| "" != *line)
        .map(|line| {
            let mut game = line.split(":");

            let id = game_id(game.next().unwrap());
            let game_max = get_game(game.next().unwrap());

            (id, game_max)
        })
        .fold(0, |mut acc, next| {

            let inc = [
                (12, "red"),
                (13, "green"),
                (14, "blue")
            ].into_iter()
                .all(|(max, col): (u32, &str)| {
                    let actual = match next.1.get(col) {
                        Some(val) => *val,
                        None => 0
                    };

                    actual <= max
                });

            acc += match inc {
                true => next.0,
                false => 0,
            };

            acc
        });

    println!("sum: {sum}");
}
fn part_2(content: &str) {
    let sum = content.split("\n")
        .filter(|line| "" != *line)
        .map(|line| {
            let mut game = line.split(":");

            let id = game_id(game.next().unwrap());
            let game_max = get_game(game.next().unwrap());

            (id, game_max)
        })
        .fold(0, |acc, (_, next)| {
            
            let power = ["red", "green", "blue"]
                .into_iter()
                .fold(
                    1,
                    |acc, key| {
                    acc * match next.get(key) {
                        Some(val) => *val,
                        None => 1
                    }
                });
            

            acc + power
        });

    println!("sum: {sum}");
}
fn main() {
    let mut f: File = File::open("input.txt").unwrap();

    let mut content = String::new();

    let _result = f.read_to_string(&mut content);

    part_1(&content);

    part_2(&content);
}