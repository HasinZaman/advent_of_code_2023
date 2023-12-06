use std::{fs::File, io::Read};

fn quad((max_time, dist): (f64, f64)) -> Option<(f64, f64)> {

    let determinant: f64 = max_time * max_time - 4. * dist;

    if determinant < 0. {
        return None;
    }

    Some(
        (
            (-1. * max_time - determinant.sqrt()) / -2.,
            (-1. * max_time + determinant.sqrt()) / -2.,
        )
    )
}

fn part_1(content: &str) {
    let (time, dist) = {
        let mut tmp = content.split("\n");

        (tmp.next().unwrap().trim(), tmp.next().unwrap().trim())
    };

    let product = time.split(":")
        .skip(1)
        .next()
        .unwrap()
        .split(" ")
        .filter(|val| *val != "")
        .map(|val| val.trim().parse::<f64>().unwrap())
        .zip(
            dist.split(":")
                .skip(1)
                .next()
                .unwrap()
                .split(" ")
                .filter(|val| *val != "")
                .map(|val| val.trim().parse::<f64>().unwrap())
        )
        .filter_map(|(max_time, dist): (f64, f64)| {

            let determinant: f64 = max_time * max_time - 4. * dist;
        
            if determinant < 0. {
                return None;
            }
        
            Some(
                (
                    (-1. * max_time - determinant.sqrt()) / -2.,
                    (-1. * max_time + determinant.sqrt()) / -2.,
                )
            )
        })
        .map(|(val_1, val_2)| {
            let (min, max) = match val_1 < val_2 {
                true => (val_1, val_2),
                false => (val_2, val_1)
            };
            // println!("{min} {max}");

            let min = match min % 1. == 0. {
                true => min + 1.,
                false => min.ceil()
            };
            let max = match max % 1. == 0. {
                true => max - 1.,
                false => max.floor()
            };

            // println!("{min} {max}");
            // println!("{}", max - min + 1.);

            max - min + 1.
        })
        .fold(1., |acc, val| acc * val);

    println!("product: {product}");
}

fn part_2(content: &str) {
    let (time, dist) = {
        let mut tmp = content.split("\n");

        (tmp.next().unwrap().trim(), tmp.next().unwrap().trim())
    };

    let time = time.split(":")
        .skip(1)
        .next()
        .unwrap()
        .split(" ")
        .filter(|val| *val != "")
        .fold(vec![], |mut acc, val| {
            acc.push(val);

            acc
        })
        .join("")
        .parse::<f64>()
        .unwrap();

    let dist = dist.split(":")
        .skip(1)
        .next()
        .unwrap()
        .split(" ")
        .filter(|val| *val != "")
        .fold(vec![], |mut acc, val| {
            acc.push(val);

            acc
        })
        .join("")
        .parse::<f64>()
        .unwrap();

    let (val_1, val_2) = {

        let determinant: f64 = time * time - 4. * dist;
    
        if determinant < 0. {
            panic!();
        }
    
        (
            (-1. * time - determinant.sqrt()) / -2.,
            (-1. * time + determinant.sqrt()) / -2.,
        )
    };

    let options = {
        let (min, max) = match val_1 < val_2 {
            true => (val_1, val_2),
            false => (val_2, val_1)
        };
        // println!("{min} {max}");

        let min = match min % 1. == 0. {
            true => min + 1.,
            false => min.ceil()
        };
        let max = match max % 1. == 0. {
            true => max - 1.,
            false => max.floor()
        };

        // println!("{min} {max}");
        // println!("{}", max - min + 1.);

        max - min + 1.
    };

    println!("options: {options}");
}

fn main() {
    let mut f: File = File::open("input.txt").unwrap();

    let mut content = String::new();

    let _result = f.read_to_string(&mut content).unwrap();

    let content = content.replace("\r", "");

    part_1(&content);

    part_2(&content);
}