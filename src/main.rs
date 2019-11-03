#![allow(unused_imports)] // TODO: find how to enable this warning only when I'm about to commit, not while I'm debugging

macro_rules! e {
    ($fmt:expr, $($e:expr),*) => { &format!(concat!("{}:{}: ", $fmt), file!(), line!(), $($e),*) };
    ($e:expr) => { &format!("{}:{}: {}", file!(), line!(), $e) };
    () => { &format!("{}:{}", file!(), line!()) };
}

//extern crate itertools;       // itertools's group_by() only works on consecutive elements!..
//use itertools::Itertools;     // misleading. should have been called "chunk_by" or something.
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

fn group_by<K, V, F>(items: &[V], get_key: F) -> HashMap<K, Vec<V>> // I somehow didn't find this in the std lib
    where K: Eq + Hash + Clone, V: Clone, F: Fn(&V) -> &K
{
    let mut result: HashMap<K, Vec<V>> = HashMap::new();
    for item in items {
        let key = get_key(item);
        result.entry(key.clone()).or_insert(Vec::new()).push(item.clone());
    }
    result
}

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
    assert_eq!(day2p1(), 6370);
    assert_eq!(day2p2(), "rmyxgdlihczskunpfijqcebtv");
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

fn has_unique_letter_count(s: &str, count: usize) -> bool {
    let groups = group_by(&s.chars().collect::<Vec<char>>(), |c| c);
    groups.into_iter().any(|(_k,g)| g.into_iter().count() == count)
}
fn day2p1() -> i64 {
    let strings: Vec<String> = strings_input(2);

    // Longass optimized version that doesn't use group_by. I wrote it first because I didn't find a group_by. 
    // let mut c2 = 0;
    // let mut c3 = 0;
    // for line in strings {
    //     let chars = &line.chars().collect::<Vec<char>>();
    //     let mut result: HashMap<&char, i64> = HashMap::new();
    //     for c in chars {
    //         let mut v = result.entry(c).or_insert(0);
    //         *v += 1;
    //     }
    //     if result.values().any(|&v| v == 2) {
    //         c2 += 1;
    //     }
    //     if result.values().any(|&v| v == 3) {
    //         c3 += 1;
    //     }
    // }

    let c2 = (&strings).into_iter().filter(|s| has_unique_letter_count(s, 2)).count() as i64;
    let c3 = (&strings).into_iter().filter(|s| has_unique_letter_count(s, 3)).count() as i64;

    println!("{}", c2 * c3);
    c2 * c3
}


fn differs_by_exactly_one(s1: &String, s2: &String) -> bool {
    let pairs = s1.chars().zip(s2.chars());
    pairs.filter(|(c1, c2)| c1 != c2).count() == 1
}
fn common_letters(s1: &String, s2: &String) -> String {
    let pairs = s1.chars().zip(s2.chars());
    pairs.filter(|(c1, c2)| c1 == c2).map(|(c,_)| c).collect()
}
fn find_pair(strings: &[String]) -> (&String, &String) {
    for s1 in strings {
        for s2 in strings {
            if differs_by_exactly_one(s1, s2) {
                return (s1, s2)
            }
        }
    }
    panic!();
}
fn day2p2() -> String {
    let strings = strings_input(2);
    let (s1, s2) = find_pair(&strings);
    let common_letters = common_letters(s1, s2);
    println!("{}", common_letters);
    common_letters
}



// TODO: setup rust-fmt with vscode


// #[cfg(test)]
// #[test]
// fn test1() {
//     assert_eq!(2 + 2, 4);
// }
