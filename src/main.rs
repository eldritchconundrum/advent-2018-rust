fn main() {
    println!("Hello, world!");
    assert_eq!(2 + 3, 4);
}

#[cfg(test)]

#[test]
fn test1() {
    assert_eq!(2 + 3, 4);
}
