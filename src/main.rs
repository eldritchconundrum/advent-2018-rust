#![allow(unused_imports)] // TODO: find how to enable this warning only when I'm about to commit, not while I'm debugging

// how comes unwrap doesn't do this by default?
macro_rules! e {
    ($fmt:expr, $($e:expr),*) => { &format!(concat!("{}:{}: ", $fmt), file!(), line!(), $($e),*) };
    ($e:expr) => { &format!("{}:{}: {}", file!(), line!(), $e) };
    () => { &format!("{}:{}", file!(), line!()) };
}

// how comes we can't derive Display? Debug is a fine default.
macro_rules! implement_display_as_debug {
    ($type: ty) => {
        impl ::std::fmt::Display for $type {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
}

// we need this macro to bypass the "helpful" compiler error in fmt_!():
// "attempted to repeat an expression containing no syntax variables matched as repeating at this depth"
macro_rules! fmt_internal_helper {
    ($e:expr) => {
        "{}"
    };
}
// with this, no need for all those format strings that are just "{}"
macro_rules! fmt_ {
    ($($e:expr),*) => {
        format!(
            concat!( $( fmt_internal_helper!($e), )* "" ),
            $($e),*
            )
    }
}
//TODO: make my own version of print!, write!, println!, writeln!...

// ---------------------------------------------------------------------------------------------------------------------

extern crate regex;
//extern crate itertools;       // itertools's group_by() only works on consecutive elements!..
//use itertools::Itertools;     // misleading. should have been called "chunk_by" or something.
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

// I somehow didn't find this in the std lib
fn group_by<K, V, F>(items: &[V], get_key: F) -> HashMap<K, Vec<V>>
where
    K: Eq + Hash + Clone,
    V: Clone,
    F: Fn(&V) -> &K,
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

use std::fmt;
use std::ops;

#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    x: i64,
    y: i64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //write!(f, "{},{}", self.x, self.y)
        write!(f, "{}", fmt_!(self.x, self.y))
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl ops::Sub<Point> for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl ops::Mul<i64> for Point {
    type Output = Point;
    fn mul(self, rhs: i64) -> Point {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl ops::Neg for Point {
    type Output = Point;
    fn neg(self) -> Point {
        Point { x: -self.x, y: -self.y }
    }
}
impl Point {
    pub fn new(x: i64, y: i64) -> Point {
        Point { x: x, y: y }
    }
    pub fn zero() -> Point {
        Point { x: 0, y: 0 }
    }
    pub fn taxi_distance(&self, p: Point) -> i64 {
        (p.x - self.x).abs() + (p.y - self.y).abs()
    }
    pub fn neighbors4(&self) -> Vec<Point> {
        vec![
            Point::new(self.x + 1, self.y),
            Point::new(self.x - 1, self.y),
            Point::new(self.x, self.y + 1),
            Point::new(self.x, self.y - 1),
        ]
    }
    pub fn neighbors8(&self) -> Vec<Point> {
        vec![
            Point::new(self.x + 1, self.y + 1),
            Point::new(self.x - 1, self.y + 1),
            Point::new(self.x + 1, self.y - 1),
            Point::new(self.x - 1, self.y - 1),
            Point::new(self.x + 1, self.y),
            Point::new(self.x - 1, self.y),
            Point::new(self.x, self.y + 1),
            Point::new(self.x, self.y - 1),
        ]
    }
    pub fn points_at_distance_at_most(&self, distance: i64) -> Vec<Point> {
        let mut vec: Vec<Point> = Vec::new();
        for dx in (-distance)..(distance + 1) {
            for dy in (-distance)..(distance + 1) {
                if dx.abs() + dy.abs() <= distance {
                    vec.push(Point {
                        x: self.x + dx,
                        y: self.y + dy,
                    });
                }
            }
        }
        vec
    }
}

// ---------------------------------------------------------------------------------------------------------------------

pub struct Matrix<T>
where
    T: Clone,
{
    x_size: usize,
    y_size: usize,
    vec: Vec<Vec<T>>,
}
impl<T> Matrix<T>
where
    T: Clone,
{
    // pub fn from_func(x_size: usize, y_size: usize, init: Fn() -> T) -> Matrix<T> {
    //     Matrix {
    //         x_size: x_size,
    //         y_size: y_size,
    //         vec: vec![vec![init(); y_size]; x_size],
    //     }
    // }
    pub fn new(x_size: usize, y_size: usize, default_value: T) -> Matrix<T> {
        Matrix {
            x_size: x_size,
            y_size: y_size,
            vec: vec![vec![default_value; y_size]; x_size],
        }
    }
}
//impl Iterable<(Point, T)>

//

//

// ---------------------------------------------------------------------------------------------------------------------

fn main() {
    // println!("{}", std::env::current_dir().expect(e!()).to_str().expect(e!()));
    assert_eq!(day1p1(), 516);
    assert_eq!(day1p2(), 71892);
    assert_eq!(day2p1(), 6370);
    assert_eq!(day2p2(), "rmyxgdlihczskunpfijqcebtv");
    //assert_eq!(day3p1(), 120408);
}

// day 1 ---------------------------------------------------------------------------------------------------------------

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

// day 2 ---------------------------------------------------------------------------------------------------------------

fn has_unique_letter_count(s: &str, count: usize) -> bool {
    let groups = group_by(&s.chars().collect::<Vec<char>>(), |c| c);
    groups.into_iter().any(|(_k, g)| g.into_iter().count() == count)
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
    pairs.filter(|(c1, c2)| c1 == c2).map(|(c, _)| c).collect()
}
fn find_pair(strings: &[String]) -> (&String, &String) {
    for s1 in strings {
        for s2 in strings {
            if differs_by_exactly_one(s1, s2) {
                return (s1, s2);
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

// day 3 ---------------------------------------------------------------------------------------------------------------

implement_display_as_debug!(Rect);
#[derive(Debug)]
struct Rect {
    id: i64,
    x: i64,
    y: i64,
    w: i64,
    h: i64,
}
fn parse(line: String) -> Rect {
    let re = Regex::new(r"^([\d]+) @ (\d+),(\d+): (\d+)x(\d+)$").expect(e!());
    let m = re.captures(&line).expect(e!());
    Rect {
        id: m.get(1).expect(e!()).as_str().parse().expect(e!()),
        x: m.get(2).expect(e!()).as_str().parse().expect(e!()),
        y: m.get(3).expect(e!()).as_str().parse().expect(e!()),
        w: m.get(4).expect(e!()).as_str().parse().expect(e!()),
        h: m.get(5).expect(e!()).as_str().parse().expect(e!()),
    } // wow, I sure miss Kotlin's  val (id,x,y,w,h) = re.find(line)!!.destructured
}
// fn matrix(rects: &[Rect]) -> Matrix<i64> {
//     let mut m: Matrix<i64> = Matrix::new();
//     for rect in rects {
//         for x in (rect.x)..(rect.x + rect.w) { // upper bound exclusive
//             for y in (rect.y)..(rect.y + rect.h) { // upper bound exclusive
//                 m[(x,y)] = m[(x,y)] + 1;
//             }
//         }
//     }
//     m
// }
// fn day3p1() -> i64 {
//     let rects = strings_input(3).into_iter().map(parse);
//     let m = matrix(rects);
//     //m.values().count { it > 1 }
//     123
// }

// TODO: setup rust-fmt with vscode

// #[cfg(test)]
// #[test]
// fn test1() {
//     assert_eq!(2 + 2, 4);
// }
