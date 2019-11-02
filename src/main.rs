#![allow(unused_imports)] // TODO: find how to enable this warning only when I'm about to commit, not while I'm debugging

macro_rules! e {
    ($fmt:expr, $($e:expr),*) => { &format!(concat!("{}:{}: ", $fmt), file!(), line!(), $($e),*) };
    ($e:expr) => { &format!("{}:{}: {}", file!(), line!(), $e) };
    () => { &format!("{}:{}", file!(), line!()) };
}

use std::collections::HashMap;
use std::collections::HashSet;

fn strings_input(i: i64) -> Vec<String> {
    use std::fs::File;
    use std::io::{prelude::*, BufReader};
    let filename = &format!("inputs-2018/input{}", i);
    let file = File::open(filename).expect(e!(filename));
    let reader = BufReader::new(file);
    let v = reader.lines().map(|s| s.expect(e!())).collect();
    v
}

// ---------------------------------------------------------------------------------------------------------------------

fn main() {
    // println!("{}", std::env::current_dir().expect(e!()).to_str().expect(e!()));
    assert_eq!(day1p1(), 516);
    assert_eq!(day1p2(), 71892);
}

fn day1p1() -> i64 {
    let numbers: Vec<i64> = strings_input(1).into_iter().map(|s| s.parse().expect(e!())).collect();
    println!("{}", numbers.iter().sum::<i64>());
    numbers.iter().sum()
}

fn day1p2() -> i64 {
    let numbers: Vec<i64> = strings_input(1).into_iter().map(|s| s.parse().expect(e!())).collect();
    let mut seen = HashSet::<i64>::new();
    let mut s = 0;
    loop {
        for n in numbers.iter() {
            seen.insert(s);
            //println!("{} + {} = {}", s, n, s + n);
            s += n;
            if seen.contains(&s) {
                println!("already seen: {}", s);
                return s;
            }
        }
    }
}


// for i in &mut v {


// TODO: setup rust-fmt with vscode


// #[cfg(test)]
// #[test]
// fn test1() {
//     assert_eq!(2 + 2, 4);
// }
