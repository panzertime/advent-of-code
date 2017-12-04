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

// change this loop to "while step <= target"
//    for mut step in (1..target) {
//        // this can be done by tracking side length and counting in the various directions one at a time
//        // 1, 1, 2, 2, 3, 3, 4, 4, 5, 5 etc.
//        for _substep in (1..len) {
//            step = step + 1;
//            x = x + 1;
//            println!("({},{})", x, y);
//        }
//        for _substep in (1..len) {
//            step = step + 1;
//            y = y + 1;
//            println!("({},{})", x, y);
//        }
//        len = len + 1;
//        for _substep in (1..len) {
//            step = step + 1;
//            x = x - 1;
//            println!("({},{})", x, y);
//        }
//        for _substep in (1..len) {
//            step = step + 1;
//            y = y -1;
//            println!("({},{})", x, y);
//        }
//    }
//    println!("Final Manhattan distance: {}", manhattan(x, y));

}

fn manhattan(x: i32, y: i32) -> i32 {
    return x.abs() + y.abs();
}
