//use crate::utils::*;
use parse_display::Display;
use std::ops;

#[derive(Display, Clone, Copy, Debug, Default)]
#[display("{x},{y}")]
pub struct Point {
    x: i64,
    y: i64,
}

impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Sub<Point> for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl ops::Mul<i64> for Point {
    type Output = Point;
    fn mul(self, rhs: i64) -> Point {
        Point::new(self.x * rhs, self.y * rhs)
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
    pub fn x(&self) -> i64 {
        self.x
    }
    pub fn y(&self) -> i64 {
        self.y
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
