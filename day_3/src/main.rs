use std::{fs::File, io::Read};

mod part_1 {
    #[derive(Debug, Clone, Copy)]
    enum Schema {
        Blank,
        Id(u32),
        Symbol,
    }

    enum Mode {
        Passive,
        Active(bool, u32),
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

    fn get_point<'a>(schema: &'a Vec<Vec<Schema>>, x: usize, y: usize) -> Option<&'a Schema> {
        let Some(row) = schema.get(y) else {
            return None;
        };

        return row.get(x);
    }

    fn neighbor_symbol(schema: &Vec<Vec<Schema>>, x: usize, y: usize) -> bool {
        let coord = (-1..2).flat_map(|x| (-1..2).map(move |y| (y, x)));

        // println!("{x} {y}");
        for (x1, y1) in coord {
            let x = match x1 < 0 as i32 {
                true => match x.checked_sub(x1.abs() as usize) {
                    Some(val) => val,
                    None => continue,
                },
                false => match x.checked_add(x1 as usize) {
                    Some(val) => val,
                    None => continue,
                },
            };
            let y = match y1 < 0 as i32 {
                true => match y.checked_sub(y1.abs() as usize) {
                    Some(val) => val,
                    None => continue,
                },
                false => match y.checked_add(y1 as usize) {
                    Some(val) => val,
                    None => continue,
                },
            };

            // println!("\t{x} {y}");

            let neighbor = get_point(schema, x, y);

            if let Some(Schema::Symbol) = neighbor {
                // println!("We got a hit");
                return true;
            }
        }

        false
    }

    pub fn main(content: &str) {
        let schema = generate_schema(content);

        let length = schema[0].len();
        let height = schema.len();

        let coord = (0..height).flat_map(|x| (0..length).map(move |y| (y, x)));

        let mut mode = Mode::Passive;
        let mut ids = Vec::new();
        for (x, y) in coord {
            let node = get_point(&schema, x, y).unwrap();

            mode = match (mode, &node, (x + 1) == length) {
                // things are looking up
                (Mode::Passive, &Schema::Id(value), false) => {
                    Mode::Active(neighbor_symbol(&schema, x, y), *value)
                }

                //just living life
                (Mode::Active(cond, id), &Schema::Id(value), false) => {
                    Mode::Active(cond || neighbor_symbol(&schema, x, y), id * 10 + value)
                }

                //good ending
                (Mode::Active(cond, id), &Schema::Id(value), true) => {
                    if cond || neighbor_symbol(&schema, x, y) {
                        ids.push(id * 10 + value);
                    }

                    Mode::Passive
                }
                (Mode::Passive, &Schema::Id(id), true) => {
                    if neighbor_symbol(&schema, x, y) {
                        ids.push(*id);
                    }
                    Mode::Passive
                }
                (Mode::Active(cond, id), _, _) => {
                    if cond {
                        ids.push(id);
                    }
                    Mode::Passive
                }

                //neutral ending
                _ => Mode::Passive,
            };
        }
        let sum = ids.iter().fold(0, |acc, val| acc + val);
        println!("ids:{ids:?}");
        println!("sum: {sum:?}");
    }
}

mod part_2 {
    use std::collections::{HashSet, HashMap};

    #[derive(Debug, Clone)]
    enum Schema {
        Blank,
        Id(u32),
        Gear(Vec<u32>),
    }

    enum Mode {
        Passive,
        Active(HashSet<(usize, usize)>, u32),
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
                            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                                Schema::Id(val.parse().unwrap())
                            }
                            "*" => Schema::Gear(vec![]),
                            _ => Schema::Blank,
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

    fn add_neighbor_gears(
        schema: &Vec<Vec<Schema>>,
        mut gears_set: HashSet<(usize, usize)>,
        x: usize,
        y: usize,
    ) -> HashSet<(usize, usize)> {
        let coord = (-1..2).flat_map(|x| (-1..2).map(move |y| (y, x)));

        // println!("{x} {y}");
        for (x1, y1) in coord {
            if (0, 0) == (x1, y1) {
                continue;
            }

            let x = match x1 < 0 as i32 {
                true => match x.checked_sub(x1.abs() as usize) {
                    Some(val) => val,
                    None => continue,
                },
                false => match x.checked_add(x1 as usize) {
                    Some(val) => val,
                    None => continue,
                },
            };
            let y = match y1 < 0 as i32 {
                true => match y.checked_sub(y1.abs() as usize) {
                    Some(val) => val,
                    None => continue,
                },
                false => match y.checked_add(y1 as usize) {
                    Some(val) => val,
                    None => continue,
                },
            };
            // println!("\t{x} {y}");

            let neighbor = get_point(schema, x, y);

            if let Some(Schema::Gear(_)) = neighbor {
                gears_set.insert((x, y));
            };
        }

        gears_set
    }

    pub fn main(content: &str) {
        let schema = generate_schema(content);

        let length = schema[0].len();
        let height = schema.len();

        let coord = (0..height).flat_map(|x| (0..length).map(move |y| (y, x)));

        let mut mode = Mode::Passive;
        let mut gear_bois: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
        for (x, y) in coord {
            let node = get_point(&schema, x, y).unwrap();

            mode = match (mode, &node, (x + 1) == length) {
                // things are looking up
                (Mode::Passive, &Schema::Id(value), false) => {
                    Mode::Active(add_neighbor_gears(&schema, HashSet::new(), x, y), *value)
                }

                //just living life
                (Mode::Active(gears, id), &Schema::Id(value), false) => {
                    Mode::Active(add_neighbor_gears(&schema, gears, x, y), id * 10 + value)
                }

                //good ending
                (Mode::Active(gears, id), &Schema::Id(value), true) => {
                    let gears = add_neighbor_gears(&schema, gears, x, y);
                    let id = id * 10 + value;
                    gears.iter()
                        .for_each(|gear| {
                            match gear_bois.contains_key(gear) {
                                true => {
                                    gear_bois.get_mut(gear).unwrap().push(id);
                                }
                                false => {
                                    gear_bois.insert(*gear, vec![id]);
                                }
                            }
                        });

                    Mode::Passive
                }
                (Mode::Passive, &Schema::Id(id), true) => {
                    let gears = add_neighbor_gears(&schema, HashSet::new(), x, y);

                    gears.iter()
                        .for_each(|gear| {
                            match gear_bois.contains_key(gear) {
                                true => {
                                    gear_bois.get_mut(gear).unwrap().push(*id);
                                }
                                false => {
                                    gear_bois.insert(*gear, vec![*id]);
                                }
                            }
                        });
                    
                    Mode::Passive
                }
                (Mode::Active(gears, id), _, _) => {
                    
                    gears.iter()
                        .for_each(|gear| {
                            match gear_bois.contains_key(gear) {
                                true => {
                                    gear_bois.get_mut(gear).unwrap().push(id);
                                }
                                false => {
                                    gear_bois.insert(*gear, vec![id]);
                                }
                            }
                        });

                    Mode::Passive
                }

                //neutral ending
                _ => Mode::Passive,
            };
        }
        let sum = gear_bois.iter()
            .filter(|(_, ids)| ids.len() == 2)
            .map(|(_, ids)| ids[0] * ids[1])
            .fold(0, |acc, val| acc + val);
        // println!("gear bois:{gear_bois:?}");
        println!("Sum: {sum}");
        // println!("sum: {sum:?}");
    }
}

fn main() {
    let mut f: File = File::open("input.txt").unwrap();

    let mut content = String::new();

    let _result = f.read_to_string(&mut content);

    part_1::main(&content);

    part_2::main(&content);
}
