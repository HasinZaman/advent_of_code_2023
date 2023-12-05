use std::{collections::{HashMap, VecDeque}, fs::File, io::Read, cmp::min};

fn part_1(content: &str) {
    let sum = content
        .split("\n")
        .filter(|line| *line != "")
        .map(|line| {
            // println!("line:{line}");
            let (winnings, hand) = {
                let mut data = line.split(":").skip(1).next().unwrap().split("|");

                (data.next().unwrap(), data.next().unwrap())
            };
            
            let mut winnings = winnings
                .trim()
                .split(" ")
                .map(|val| val.trim())
                .filter(|val| *val != "")
                .map(|val| (val.parse::<u32>().unwrap(), 0))
                .collect::<HashMap<u32, u32>>();
            hand.trim()
                .split(" ")
                .map(|val| val.trim())
                .filter(|val| *val != "")
                .map(|val| {
                    // println!("val: {val}");
                    val.parse::<u32>().unwrap()
                })
                .for_each(|key| match winnings.contains_key(&key) {
                    true => {
                        let tmp = winnings.get(&key).unwrap();

                        winnings.insert(key, tmp + 1);
                    }
                    false => {}
                });

            winnings.iter().fold(0, |acc, (_key, hits)| acc + hits)
        })
        .map(|hits| match hits {
            0 => 0,
            value => 2_u32.pow(value - 1),
        })
        .fold(0, |acc, val| acc + val);

    println!("sum: {sum}");
}

fn part_2(content: &str) {
    let sum = content
        .split("\n")
        .filter(|line| *line != "")
        .map(|line| {
            // println!("line:{line}");
            let (winnings, hand) = {
                let mut data = line.split(":").skip(1).next().unwrap().split("|");

                (data.next().unwrap(), data.next().unwrap())
            };
            
            let mut winnings = winnings
                .trim()
                .split(" ")
                .map(|val| val.trim())
                .filter(|val| *val != "")
                .map(|val| (val.parse::<u32>().unwrap(), 0))
                .collect::<HashMap<u32, u32>>();
            hand.trim()
                .split(" ")
                .map(|val| val.trim())
                .filter(|val| *val != "")
                .map(|val| {
                    // println!("val: {val}");
                    val.parse::<u32>().unwrap()
                })
                .for_each(|key| match winnings.contains_key(&key) {
                    true => {
                        let tmp = winnings.get(&key).unwrap();

                        winnings.insert(key, tmp + 1);
                    }
                    false => {}
                });

            winnings.iter().fold(0, |acc, (_key, hits)| acc + hits)
        })
        .fold((0, VecDeque::new()), |(sum, mut queue), wins| {
            // println!("queue:{queue:?}");
            let cards = match queue.pop_front() {
                Some(copies) => copies + 1,
                None => 1
            };

            // println!("cards: {cards} => wins: {wins}\nsum:{sum}\n");

            for index in 0..(min(wins as usize, queue.len()))  {
                queue[index as usize] += cards; 
            }
            for _ in 0..((wins as usize).checked_sub(queue.len()).unwrap_or_else(|| 0))  {
                queue.push_back(cards); 
            }

            (sum + cards, queue)
        });

    println!("sum: {sum:?}");
}

fn main() {
    let mut f: File = File::open("input.txt").unwrap();

    let mut content = String::new();

    let _result = f.read_to_string(&mut content);

    part_1(&content);

    part_2(&content);
}
