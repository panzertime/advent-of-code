use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let f = File::open(&args[1]).unwrap();
    let file = BufReader::new(&f);

    let mut checksum = 0;

    for line in file.lines() {
        let l = line.unwrap();
        let row = l.split_whitespace();

        let mut cells = Vec::<i32>::new();

        for cell in row {
            cells.push(cell.parse::<i32>().unwrap());
        }
        cells.sort_unstable();

        //checksum = checksum + sum(cells.pop().unwrap() - cells.first().unwrap());
        checksum = checksum + sumrow(cells);

    }

    print!("Spreadsheet checksum is {}", checksum);
}

fn sumrow(v : Vec<i32>) -> i32{
    for f in &v {
        for l in &v {
            if (l % f) == 0 && l != f{
                return l / f
            } 
        }
    }
    return 0
}