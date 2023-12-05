use std::{fs::File, io::Read};

#[derive(Debug, Clone, Copy)]
enum Schema {
    Blank,
    Id(u32),
    Symbol,
}


#[derive(Debug, Clone)]
enum GearSchema {
    Blank,
    Id(u32),
    Gear(Vec<u32>),
}


enum Mode{
    Passive,
    Active(bool, u32)
}


fn generate_schema(content: &str) -> Vec<Vec<Schema>> {
    content
        .split("\n")
        .filter(|val| *val != "")
        .map(|line| {
            line.trim()
                .chars()
                .map(|val| {
                    let val = val.to_string();
                    let val = val.as_str();

                    match val {
                        "." => Schema::Blank,
                        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                            Schema::Id(val.parse().unwrap())
                        }
                        _ => Schema::Symbol,
                    }
                })
                .collect()
        })
        .collect()
}

fn generate_gear_schema(content: &str) -> Vec<Vec<GearSchema>> {
    content
        .split("\n")
        .filter(|val| *val != "")
        .map(|line| {
            line.trim()
                .chars()
                .map(|val| {
                    let val = val.to_string();
                    let val = val.as_str();

                    match val {
                        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                            GearSchema::Id(val.parse().unwrap())
                        }
                        "*" => GearSchema::Gear(Vec::new()),
                        _ => GearSchema::Blank,
                    }
                })
                .collect()
        })
        .collect()
}

fn get_point<'a>(schema: &'a Vec<Vec<Schema>>, x: usize, y: usize) -> Option<&'a Schema> {
    let Some(row) = schema.get(y) else {
        return None;
    };

    return row.get(x);
}

fn neighbor_symbol(schema: & Vec<Vec<Schema>>, x: usize, y: usize) -> bool {
    let coord = (-1..2)
    .flat_map(|x| {
        (-1..2)
            .map(move |y| (y, x))
    });

    // println!("{x} {y}");
    for (x1, y1) in coord {
        let x = match x1 < 0 as i32 {
            true => {
                match x.checked_sub(x1.abs() as usize) {
                    Some(val) => val,
                    None => continue,
                }
            },
            false => {
                match x.checked_add(x1 as usize) {
                    Some(val) => val,
                    None => continue,
                }
            }
        };
        let y = match y1 < 0 as i32 {
            true => {
                match y.checked_sub(y1.abs() as usize) {
                    Some(val) => val,
                    None => continue,
                }
            },
            false => {
                match y.checked_add(y1 as usize) {
                    Some(val) => val,
                    None => continue,
                }
            }
        };

        // println!("\t{x} {y}");

        let neighbor = get_point(
            schema,
            x,
            y
        );

        if let Some(Schema::Symbol) = neighbor {
            // println!("We got a hit");
            return true
        }
    }

    false
}

fn part_1(content: &str) {
    let schema = generate_schema(content);

    let length = schema[0].len();
    let height = schema.len();

    let coord = (0..height)
        .flat_map(|x| {
            (0..length)
                .map(move |y| (y, x))
        });

    let mut mode = Mode::Passive;
    let mut ids = Vec::new();
    for (x, y) in coord {
        let node = get_point(&schema, x, y).unwrap();

        mode = match (mode, &node, (x+1)==length) {
            // things are looking up
            (Mode::Passive, &Schema::Id(value), false) => {
                Mode::Active(neighbor_symbol(&schema, x, y), *value)
            },

            //just living life
            (Mode::Active(cond, id), &Schema::Id(value), false) => {
                Mode::Active(cond || neighbor_symbol(&schema, x, y), id * 10 + value)
            },

            //good ending
            (Mode::Active(cond, id), &Schema::Id(value), true) => {
                if cond || neighbor_symbol(&schema, x, y) {
                    ids.push(id * 10 + value);
                }

                Mode::Passive
            },
            (Mode::Passive, &Schema::Id(id), true) => {
                if neighbor_symbol(&schema, x, y) {
                    ids.push(*id);
                }
                Mode::Passive
            },
            (Mode::Active(cond, id), _, _) => {
                if cond {
                    ids.push(id);
                }
                Mode::Passive
            },

            //neutral ending
            _ => Mode::Passive
        };
    }
    let sum = ids.iter()
        .fold(0, |acc, val| acc + val);
    println!("ids:{ids:?}");
    println!("sum: {sum:?}");
}
fn part_2(_content: &str) {}

fn main() {
    let mut f: File = File::open("input.txt").unwrap();

    let mut content = String::new();

    let _result = f.read_to_string(&mut content);

    part_1(&content);

    part_2(&content);
}
