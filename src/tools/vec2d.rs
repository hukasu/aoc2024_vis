use std::ops::{Index, IndexMut};

use crate::tools::Coord;

#[derive(Debug)]
pub struct Vec2d<'a, T> {
    data: &'a mut [T],
    width: usize,
    height: usize,
}

impl<'a, T> Vec2d<'a, T> {
    pub fn new(data: &'a mut [T], width: usize, height: usize) -> Vec2d<'a, T> {
        Self {
            data,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl<T> Index<Coord> for Vec2d<'_, T> {
    type Output = T;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.data[index.row * self.width + index.column]
    }
}

impl<T> IndexMut<Coord> for Vec2d<'_, T> {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self.data[index.row * self.width + index.column]
    }
}
