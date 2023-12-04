use std::{fs::File, io::Read};


fn part_1(content: &str) {

}
fn part_2(content: &str) {
    
}

enum CellType {
    Part,
    Empty,
    Symbol
}

fn main() {
    let mut f: File = File::open("input.txt").unwrap();

    let mut content = String::new();

    let _result = f.read_to_string(&mut content);

    part_1(&content);

    part_2(&content);
}
