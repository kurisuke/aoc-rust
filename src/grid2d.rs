use itertools::concat;
use std::fmt::{self, Display, Formatter};
use std::iter::Zip;

#[derive(Clone)]
pub struct Grid2D<T> {
    el: Vec<T>,
    width: usize,
    height: usize,
}

#[derive(Debug, Clone)]
pub struct Grid2DError;

#[allow(dead_code)]
pub enum Wrap {
    None,
    WrapX,
    WrapY,
    WrapXY,
}

pub struct Coords {
    pub x: i64,
    pub y: i64,
}

impl Grid2D<char> {
    pub fn new(input: &str) -> Result<Grid2D<char>, Grid2DError> {
        let line_lens: Vec<_> = input.lines().map(|l| l.len()).collect();
        if line_lens.iter().min() != line_lens.iter().max() {
            Err(Grid2DError {})
        } else {
            let width = *line_lens.iter().min().unwrap();
            let height = input.lines().count();
            let el: Vec<_> = concat(input.lines().map(|l| l.chars().collect::<Vec<_>>()));
            Ok(Grid2D { el, width, height })
        }
    }
}

impl<T> Grid2D<T> {
    pub fn at(&self, x: i64, y: i64) -> Option<&T> {
        if x < 0 || y < 0 || x >= self.width() || y >= self.height() {
            None
        } else {
            Some(&self.el[y as usize * self.width + x as usize])
        }
    }

    pub fn set(&mut self, x: i64, y: i64, v: T) -> bool {
        if x < 0 || y < 0 || x >= self.width() || y >= self.height() {
            false
        } else {
            self.el[y as usize * self.width + x as usize] = v;
            true
        }
    }

    pub fn height(&self) -> i64 {
        self.height as i64
    }

    pub fn width(&self) -> i64 {
        self.width as i64
    }

    pub fn neighbors(&self, x: i64, y: i64) -> Vec<Option<&T>> {
        vec![
            self.at(x, y - 1),
            self.at(x + 1, y - 1),
            self.at(x + 1, y),
            self.at(x + 1, y + 1),
            self.at(x, y + 1),
            self.at(x - 1, y + 1),
            self.at(x - 1, y),
            self.at(x - 1, y - 1),
        ]
    }

    pub fn iter(&self) -> Iter<T> {
        Iter::new(&self)
    }

    #[allow(dead_code)]
    pub fn coords_iter(&self) -> CoordsIter<T> {
        CoordsIter::new(&self)
    }

    pub fn enumerate(&self) -> Zip<CoordsIter<T>, Iter<T>> {
        CoordsIter::new(&self).zip(Iter::new(&self))
    }

    #[allow(dead_code)]
    pub fn traverse(&self, d_x: i64, d_y: i64) -> TraverseIter<T> {
        TraverseIter::new(&self, 0, 0, d_x, d_y, Wrap::None)
    }

    pub fn traverse_wrap(&self, d_x: i64, d_y: i64, wrap: Wrap) -> TraverseIter<T> {
        TraverseIter::new(&self, 0, 0, d_x, d_y, wrap)
    }

    pub fn traverse_init_wrap(
        &self,
        init_x: i64,
        init_y: i64,
        d_x: i64,
        d_y: i64,
        wrap: Wrap,
    ) -> TraverseIter<T> {
        TraverseIter::new(&self, init_x, init_y, d_x, d_y, wrap)
    }
}

impl<T: std::cmp::PartialEq> Grid2D<T> {
    pub fn count(&self, v: T) -> usize {
        self.iter().filter(|&x| x == &v).count()
    }
}

impl Display for Grid2D<char> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut s = String::from("");
        for (idx, v) in self.el.iter().enumerate() {
            if idx > 0 && idx % self.width == 0 {
                s += "\n";
            }
            s.push(*v);
        }
        write!(f, "{}", s)
    }
}

pub struct Iter<'a, T> {
    grid: &'a Grid2D<T>,
    cur_x: i64,
    cur_y: i64,
}

impl<'a, T> Iter<'a, T> {
    fn new(grid: &'a Grid2D<T>) -> Iter<T> {
        Iter {
            grid,
            cur_x: 0,
            cur_y: 0,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let ret = self.grid.at(self.cur_x, self.cur_y);
        if self.cur_x + 1 < self.grid.width() {
            self.cur_x += 1;
        } else {
            self.cur_x = 0;
            self.cur_y += 1;
        }
        ret
    }
}

pub struct CoordsIter<'a, T> {
    grid: &'a Grid2D<T>,
    cur_x: i64,
    cur_y: i64,
}

impl<'a, T> CoordsIter<'a, T> {
    fn new(grid: &'a Grid2D<T>) -> CoordsIter<T> {
        CoordsIter {
            grid,
            cur_x: 0,
            cur_y: 0,
        }
    }
}

impl<'a, T> Iterator for CoordsIter<'a, T> {
    type Item = Coords;

    fn next(&mut self) -> Option<Coords> {
        let ret = if self.cur_x < 0
            || self.cur_y < 0
            || self.cur_x >= self.grid.width()
            || self.cur_y >= self.grid.height()
        {
            None
        } else {
            Some(Coords {
                x: self.cur_x,
                y: self.cur_y,
            })
        };
        if self.cur_x + 1 < self.grid.width() {
            self.cur_x += 1;
        } else {
            self.cur_x = 0;
            self.cur_y += 1;
        }
        ret
    }
}

pub struct TraverseIter<'a, T> {
    grid: &'a Grid2D<T>,
    cur_x: i64,
    cur_y: i64,
    d_x: i64,
    d_y: i64,
    wrap: Wrap,
}

impl<'a, T> TraverseIter<'a, T> {
    fn new(
        grid: &'a Grid2D<T>,
        init_x: i64,
        init_y: i64,
        d_x: i64,
        d_y: i64,
        wrap: Wrap,
    ) -> TraverseIter<'a, T> {
        TraverseIter {
            grid,
            cur_x: init_x,
            cur_y: init_y,
            d_x,
            d_y,
            wrap,
        }
    }
}

impl<'a, T> Iterator for TraverseIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let ret = self.grid.at(self.cur_x, self.cur_y);
        match self.wrap {
            Wrap::None => {
                self.cur_x += self.d_x;
                self.cur_y += self.d_y;
            }
            Wrap::WrapX => {
                self.cur_x = (self.cur_x + self.d_x) % self.grid.width();
                self.cur_y += self.d_y;
            }
            Wrap::WrapY => {
                self.cur_x += self.d_x;
                self.cur_y = (self.cur_y + self.d_y) % self.grid.height();
            }
            Wrap::WrapXY => {
                self.cur_x = (self.cur_x + self.d_x) % self.grid.width();
                self.cur_y = (self.cur_y + self.d_y) % self.grid.height();
            }
        }
        ret
    }
}
