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

    pub fn at(&self, x: usize, y: usize) -> Option<char> {
        if x >= self.width() || y >= self.height() {
            None
        } else {
            Some(self.el[y][x])
        }
    }

    pub fn height(&self) -> usize {
        self.el.len()
    }

    pub fn width(&self) -> usize {
        if self.el.is_empty() {
            0
        } else {
            self.el[0].len()
        }
    }

    #[allow(dead_code)]
    pub fn traverse(&self, d_x: usize, d_y: usize) -> TraverseIter {
        TraverseIter::new(&self, 0, 0, d_x, d_y, Wrap::None)
    }

    pub fn traverse_wrap(&self, d_x: usize, d_y: usize, wrap: Wrap) -> TraverseIter {
        TraverseIter::new(&self, 0, 0, d_x, d_y, wrap)
    }

    #[allow(dead_code)]
    pub fn traverse_init_wrap(
        &self,
        init_x: usize,
        init_y: usize,
        d_x: usize,
        d_y: usize,
        wrap: Wrap,
    ) -> TraverseIter {
        TraverseIter::new(&self, init_x, init_y, d_x, d_y, wrap)
    }
}

#[allow(dead_code)]
pub enum Wrap {
    None,
    WrapX,
    WrapY,
    WrapXY,
}

pub struct TraverseIter<'a> {
    grid: &'a Grid2D,
    cur_x: usize,
    cur_y: usize,
    d_x: usize,
    d_y: usize,
    wrap: Wrap,
}

impl<'a> TraverseIter<'a> {
    fn new(
        grid: &'a Grid2D,
        init_x: usize,
        init_y: usize,
        d_x: usize,
        d_y: usize,
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
