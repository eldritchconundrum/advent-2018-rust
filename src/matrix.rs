use crate::point::*;
use crate::utils::*;

pub struct Matrix<T>
where
    T: Clone,
{
    vec: Vec<Vec<T>>,
}

impl<T> Matrix<T>
where
    T: Copy + Clone,
{
    pub fn x_size(&self) -> usize {
        self.vec.len()
    }
    pub fn y_size(&self) -> usize {
        self.vec[0].len()
    }
    pub fn from_func<F>(x_size: usize, y_size: usize, init: F) -> Matrix<T>
    where
        F: Fn() -> T,
    {
        assert!(x_size > 0); // or y_size will crash later
        Matrix {
            vec: vec![vec![init(); y_size]; x_size],
        }
    }
    pub fn new(x_size: usize, y_size: usize, default_value: T) -> Matrix<T> {
        Matrix {
            vec: vec![vec![default_value; y_size]; x_size],
        }
    }
    pub fn get(&self, p: Point) -> Option<T> {
        if !range(&self.vec).contains(&(p.x() as usize)) {
            return None;
        }
        let vec2 = &self.vec[p.x() as usize];
        if !range(&vec2).contains(&(p.y() as usize)) {
            return None;
        }
        Some(vec2[p.y() as usize])
    }
    pub fn set(&mut self, p: Point, item: T) {
        self.vec[p.x() as usize][p.y() as usize] = item;
    }
    // TODO: implement Index/IndexMut?
    pub fn values(&self) -> Vec<T> {
        let mut values: Vec<T> = Vec::new();
        for x in range(&self.vec) {
            for y in range(&self.vec) {
                values.push(self.vec[x][y]);
            }
        }
        values
    }
}
