use std::fmt::{self, Display, Formatter};

#[derive(Clone)]
pub struct Grid2D {
    el: Vec<Vec<char>>,
}

#[derive(Debug, Clone)]
pub struct Grid2DError;

impl Grid2D {
    pub fn new(input: &str) -> Result<Grid2D, Grid2DError> {
        let line_lens: Vec<_> = input.lines().map(|l| l.len()).collect();
        if line_lens.iter().min() != line_lens.iter().max() {
            Err(Grid2DError {})
        } else {
            let el: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
            Ok(Grid2D { el })
        }
    }

    pub fn at(&self, x: i64, y: i64) -> Option<char> {
        if x < 0 || y < 0 || x >= self.width() || y >= self.height() {
            None
        } else {
            Some(self.el[y as usize][x as usize])
        }
    }

    pub fn set(&mut self, x: i64, y: i64, v: char) -> bool {
        if x < 0 || y < 0 || x >= self.width() || y >= self.height() {
            false
        } else {
            self.el[y as usize][x as usize] = v;
            true
        }
    }

    pub fn height(&self) -> i64 {
        self.el.len() as i64
    }

    pub fn width(&self) -> i64 {
        if self.el.is_empty() {
            0
        } else {
            self.el[0].len() as i64
        }
    }

    pub fn neighbors(&self, x: i64, y: i64) -> Vec<Option<char>> {
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

    pub fn count(&self, v: char) -> usize {
        self.iter().filter(|&x| x == v).count()
    }

    pub fn iter(&self) -> Iter {
        Iter::new(&self)
    }

    #[allow(dead_code)]
    pub fn traverse(&self, d_x: i64, d_y: i64) -> TraverseIter {
        TraverseIter::new(&self, 0, 0, d_x, d_y, Wrap::None)
    }

    pub fn traverse_wrap(&self, d_x: i64, d_y: i64, wrap: Wrap) -> TraverseIter {
        TraverseIter::new(&self, 0, 0, d_x, d_y, wrap)
    }

    #[allow(dead_code)]
    pub fn traverse_init_wrap(
        &self,
        init_x: i64,
        init_y: i64,
        d_x: i64,
        d_y: i64,
        wrap: Wrap,
    ) -> TraverseIter {
        TraverseIter::new(&self, init_x, init_y, d_x, d_y, wrap)
    }
}

impl Display for Grid2D {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let ss: Vec<String> = self.el.iter().map(|v| v.iter().collect()).collect();
        write!(f, "{}", ss.join("\n"))
    }
}

#[allow(dead_code)]
pub enum Wrap {
    None,
    WrapX,
    WrapY,
    WrapXY,
}

pub struct Iter<'a> {
    grid: &'a Grid2D,
    cur_x: i64,
    cur_y: i64,
}

impl<'a> Iter<'a> {
    fn new(grid: &'a Grid2D) -> Iter {
        Iter {
            grid,
            cur_x: 0,
            cur_y: 0,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
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

pub struct TraverseIter<'a> {
    grid: &'a Grid2D,
    cur_x: i64,
    cur_y: i64,
    d_x: i64,
    d_y: i64,
    wrap: Wrap,
}

impl<'a> TraverseIter<'a> {
    fn new(
        grid: &'a Grid2D,
        init_x: i64,
        init_y: i64,
        d_x: i64,
        d_y: i64,
        wrap: Wrap,
    ) -> TraverseIter<'a> {
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

impl<'a> Iterator for TraverseIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
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
