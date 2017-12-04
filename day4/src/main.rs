use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashSet;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let f = File::open(&args[1]).unwrap();
    let file = BufReader::new(&f);

    let mut phrases = 0;

    for line in file.lines() {
        let l = line.unwrap();
        let mut memory = HashSet::new();
        let phrase = l.split_whitespace();
        let mut count = 0;

        for token in phrase {
            memory.insert(token.to_string());
            count = count + 1;
            println!("{:?}", token);
        }

        if count == memory.len() {
            phrases = phrases + 1;
        }
    }

    print!("Total count of valid passphrases is {}", phrases);
}
