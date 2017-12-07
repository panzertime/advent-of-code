use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
//use std::collections::HashSet;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let f = File::open(&args[1]).unwrap();
    let file = BufReader::new(&f);

    for line in file.lines() {
        let l = line.unwrap();
        let row = l.split_whitespace();

        let mut cells = Vec::<i32>::new();

        for cell in row {
            cells.push(cell.parse::<i32>().unwrap());
        }

        let mut memory = Vec::<Vec<i32>>::new();
        let mut steps = 0;
        let mut in_loop = false;

        loop {
            steps = steps + 1;
            let balanced = balance(cells);

            if memory.contains(&balanced) && !in_loop {
                in_loop = true;
                steps = 0;
                memory.clear();
                //println!("Loop after {} steps", steps);
                //println!("Minimum loop size is {}", steps - search(memory, balanced) - 1);
            }
            if memory.contains(&balanced) && in_loop {
                println!("Loop period is {} steps", steps);
                //println!("Minimum loop size is {}", steps - search(memory, balanced) - 1);
                return
            }
            cells = balanced.clone();
            memory.push(balanced);
        }
    }
}

fn search(space: Vec<Vec<i32>>, word: Vec<i32>) -> i32{
    for candidate in space {
        let mut flag = true;
        for i in 1..candidate.len() {
            if candidate[i-1] != word[i-1] {
                flag = false;
            }
            if flag {
                return (i - 1) as i32;
            }
        }
    }
    return 0;
}

fn balance(mut input: Vec<i32>) -> Vec<i32> {
    println!("Balancing:");
    for cell in &input {
                print!("{}\t", cell);
            }
    println!("");

    let mut head = 0;
    let mut max = 0;
    for i in 0..input.len() {
        if input[i] > max {
            head = i;
            max = input[i];
        }
    }

    let mut buf = max;
    input[head] = 0;
    head = head + 1;
    for _i in 0..max {
        buf = buf - 1;
        if head == input.len() {
            head = 0;
        }
        input[head] = input[head] + 1;
        head = head + 1;
    }

    println!("Balanced:");
    for cell in &input {
                print!("{}\t", cell);
            }
    println!("");
    println!("");

    return input;

}

