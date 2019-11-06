// If I understand correctly, main is a special place where we declare the other files' modules
mod matrix;
mod point;
mod utils;
// And also we use them obv
use matrix::*;
use point::*;
use utils::*;

// "extern crate" statements are not needed anymore because Cargo.toml is edition="2018" somehow.
#[macro_use] // except this one
extern crate lazy_static;

// ---------------------------------------------------------------------------------------------------------------------

fn strings_input(i: i64) -> Vec<String> {
    use std::fs::File;
    use std::io::{prelude::*, BufReader};
    let filename = &format!("inputs-2018/input{}", i);
    let file = File::open(filename).expect(e!(filename));
    let reader = BufReader::new(file);
    let v = reader.lines().map(|s| s.expect(e!())).collect();
    v
}

fn main() {
    // println!("{}", std::env::current_dir().expect(e!()).to_str().expect(e!()));
    assert_eq!(day1p1(), 516);
    assert_eq!(day1p2(), 71892);
    assert_eq!(day2p1(), 6370);
    assert_eq!(day2p2(), "rmyxgdlihczskunpfijqcebtv");
    assert_eq!(day3p1(), 120408);
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
    groups.into_iter().any(|(_k, g)| g.len() == count)
}
fn day2p1() -> i64 {
    let strings: Vec<String> = strings_input(2);
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
lazy_static! {
    static ref REGEX: Regex = Regex::new(r"^#([\d]+) @ (\d+),(\d+): (\d+)x(\d+)$").expect(e!());
}
fn parse(line: String) -> Rect {
    let m = REGEX.captures(&line).expect(e!(line));
    Rect {
        id: m.get(1).expect(e!()).as_str().parse().expect(e!()),
        x: m.get(2).expect(e!()).as_str().parse().expect(e!()),
        y: m.get(3).expect(e!()).as_str().parse().expect(e!()),
        w: m.get(4).expect(e!()).as_str().parse().expect(e!()),
        h: m.get(5).expect(e!()).as_str().parse().expect(e!()),
    } // wow, I sure miss Kotlin's  val (id,x,y,w,h) = re.find(line)!!.destructured
}
fn matrix(rects: &[Rect]) -> Matrix<i64> {
    let mut m: Matrix<i64> = Matrix::new(1000, 1000, 0);
    for rect in rects {
        for x in (rect.x)..(rect.x + rect.w) {
            for y in (rect.y)..(rect.y + rect.h) {
                //m[(x, y)] = m[(x, y)] + 1;
                let v = m.get(Point::new(x, y)).expect(e!(fmt_!(Point::new(x, y)))) + 1;
                m.set(Point::new(x, y), v);
            }
        }
    }
    m
}
fn day3p1() -> usize {
    let rects: &[Rect] = &strings_input(3).into_iter().map(parse).collect::<Vec<Rect>>();
    let m = matrix(rects);
    let count = m.values().into_iter().filter(|v| *v as i64 > 1 as i64).count();
    println!("{}", count);
    count
}
