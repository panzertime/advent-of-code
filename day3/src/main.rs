use std::env;
use std::num;

fn main() {
    let args: Vec<_> = env::args().collect();
    let target = args[1].parse::<i32>().unwrap();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut len = 1;
    let mut step = 1;
    let mut count = 1;

    println!("Origin at ({},{})", x, y);

    while count < target {
        // inc x len steps
        for i in (1..len) {
            x = x + step;
            count = count + 1;
            if count == target {
                println!("Final Manhattan distance: {}", manhattan(x, y));
            }
        }
        // inc y len steps
        for i in (1..len) {
            y = y + step;
            count = count + 1;
            if count == target {
                println!("Final Manhattan distance: {}", manhattan(x, y));
            }
        }
        // flip x/y direction (i.e. step = -step)
        step = -1 * step;
        // increment len
        len = len + 1;
        // continue
    }

fn manhattan(x: i32, y: i32) -> i32 {
    return x.abs() + y.abs();
}
