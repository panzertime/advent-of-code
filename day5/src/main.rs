use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    let args: Vec<_> = env::args().collect();
    let f = File::open(&args[1]).unwrap();
    let file = BufReader::new(&f);

    let mut tape = Vec::<i32>::new();

    for line in file.lines() {
        let l = line.unwrap();
        tape.push(l.parse::<i32>().unwrap());
    }

    let mut head: i32 = 0;
    let mut steps = 0;
    loop {
        let uhead: usize = head as usize;
        if head < 0 {
            println!("Number of steps: {}", steps);
            return
        }
        if uhead >= tape.len() {
            println!("Number of steps: {}", steps);
            return
        }
        let buf = tape[uhead];
        tape[uhead] = tape[uhead] + 1;
        head = head + buf;
        steps = steps + 1;
    }

}
