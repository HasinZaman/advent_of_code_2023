use std::{fs::File, io::Read, str::Split};

fn get_seeds_1(seeds: &str) -> Vec<u64> {
    seeds.split(":")
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .split(" ")
        .filter(|val| *val != "")
        .map(|val| {
            // println!("{}", val.trim());
            val.trim().parse::<u64>().unwrap()
        })
        .collect()
}

fn get_seeds_2(seeds: &str) -> Vec<u64> {
    seeds.split(":")
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .split(" ")
        .filter(|val| *val != "")
        .map(|val| {
            // println!("{}", val.trim());
            val.trim().parse::<u64>().unwrap()
        })
        .collect::<Vec<u64>>()
        .chunks(2)
        .flat_map(|val| (val[0]..(val[0]+val[1])))
        .collect()
}

fn get_maps(chunks: Split<'_, &str>) -> Vec<Vec<(u64, u64, u64)>> {
    chunks.filter(|val| *val != "")
        .map(|chunk| {
            let mut chunk: Vec<(u64, u64, u64)> = chunk.split("\n")
                .map(|line| line.trim())
                .filter(|line| *line != "")
                .skip(1)
                .map(|line| {
                    let mut tmp = line.trim()
                        .split(" ")
                        .filter(|val| *val != "")
                        .map(|val| val.trim().parse::<u64>().unwrap());

                    (tmp.next().unwrap(), tmp.next().unwrap(), tmp.next().unwrap())
                })
                .collect();

            chunk.sort_by(|a,  b| b.2.cmp(&a.2));

            chunk
        })
        .collect()
}

fn get_range(maps: &Vec<(u64, u64, u64)>, val: u64) -> u64 {
    maps.iter()
        .filter(|(_end, start, dist)| *start <= val && val < start + dist)
        .map(|(end, start, _dist)| end + (val - start))
        .next()
        .unwrap_or(val)
}

fn part_1(content: &str) {
    let mut chunks = content.split("\n\n");

    let seeds = get_seeds_1(chunks.next().unwrap());

    let maps = get_maps(chunks);

    let smallest = seeds.iter()
        .map(|seed| {
            maps.iter()
                .fold(*seed, |acc, map| get_range(map, acc))
        })
        .min();
    println!("{smallest:?}");
}

fn part_2(content: &str) {
    let mut chunks = content.split("\n\n");

    let seeds = chunks.next().unwrap();

    // println!("{seeds:?}");

    let maps = get_maps(chunks);

    let smallest = seeds.split(":")
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .split(" ")
        .filter(|val| *val != "")
        .map(|val| {
            // println!("{}", val.trim());
            val.trim().parse::<u64>().unwrap()
        })
        .collect::<Vec<u64>>()
        .chunks(2)
        .flat_map(|val| (val[0]..(val[0]+val[1])))
        .map(|seed| {
            maps.iter()
                .fold(seed, |acc, map| get_range(map, acc))
        })
        .min();
    println!("{smallest:?}");
}


fn main() {
    let mut f: File = File::open("input.txt").unwrap();

    let mut content = String::new();

    let _result = f.read_to_string(&mut content);

    let content = content.replace("\r", "");

    part_1(&content);

    part_2(&content);
}