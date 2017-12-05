use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let f = File::open(&args[1]).unwrap();
    let file = BufReader::new(&f);

    let mut tape = Vec::<i32>::new();

    for line in file.lines() {
        let l = line.unwrap();
        for character in l.to_string().chars() {
            tape.push(character.to_digit(10).unwrap() as i32);
        }
    }

    let mut sum = 0;
    
    for head in (0..(tape.len() - 1)) {
        let ohead = mod_sum(head, tape.len() / 2, tape.len());
        if tape[head] == tape[ohead] {
            sum = sum + tape[head];
        }
    }

//    if tape.pop().unwrap() == tape[0] {
//        sum = sum + tape[0];
//    }

    println!("Sum is {}", sum);

}

fn mod_sum (a : usize, b : usize, modulus : usize) -> usize {
    let s = a + b;
    return s % modulus
}