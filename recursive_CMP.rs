use std::env;
use std::cmp;

fn main() {
    let args: Vec<String> = env::args().collect();

    let n: u32 = args[1].parse::<u32>().unwrap();
    let l: u32 = args[2].parse::<u32>().unwrap();

    let mut p: Vec<i64> = Vec::new();

    for i in 3..args.len() {
        p.push(args[i].parse::<i64>().unwrap());
    }

    println!("{} {} {:?}", n, l, p);

    let coin_count: u32 = recursive_change_make(n as i64, (l - 1) as i64, &p);

    println!("{}", coin_count);
}

fn recursive_change_make(n: i64, l: i64, p: &Vec<i64>) -> u32 {
    if n == 0 {
        return 0;
    }

    else if n < 0 {
        return u32::max_value();
    }

    else if n >= 1 && l < 0 {
        return u32::max_value();
    }

    let mut index: u32 = 0;

    for i in 0..p.len() {
        if i as i64 == l {
            index = i as u32;
        }
    }

    let left: u32 = recursive_change_make(n, l - 1, &p);
    let right: u32 = recursive_change_make(n - &p[index as usize], l, &p);

    let max: u32 = <u32>::max_value();

    if right != max {
        return cmp::min(left, right + 1);
    }

    return cmp::min(left, right);
}
