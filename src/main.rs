#![allow(unused_imports)] // TODO: find how to enable this warning only when I'm about to commit, not while I'm debugging

macro_rules! unwrap {
    ( $x:expr ) => { $x.expect(&format!("line {} in {}", line!(), file!())) }
}

fn main() {
    assert_eq!(day1p1(), 516);
}

use std::fs::File;
use std::io::{prelude::*, BufReader};

fn day1p1() -> i64 {
    println!("{}", unwrap!(unwrap!(std::env::current_dir()).to_str()));
    let filename = "inputs-2018/input1";
    let file = unwrap!(File::open(filename));
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines().map(|s| unwrap!(s)) {
        //println!("{}", line);
        let n = unwrap!(line.parse::<i64>());
        sum += n;
    }
    sum
}



// TODO: setup rust-fmt with vscode


// #[cfg(test)]
// #[test]
// fn test1() {
//     assert_eq!(2 + 2, 4);
// }
