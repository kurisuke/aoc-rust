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

#[derive(Copy, Clone, Debug)]
pub enum Flip {
    FlipNone,
    FlipH,
    FlipV,
}

#[derive(Clone)]
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
    pub fn at(&self, c: &Coords) -> Option<&T> {
        if c.x < 0 || c.y < 0 || c.x >= self.width() || c.y >= self.height() {
            None
        } else {
            Some(&self.el[c.y as usize * self.width + c.x as usize])
        }
    }

    pub fn set(&mut self, c: &Coords, v: T) -> bool {
        if c.x < 0 || c.y < 0 || c.x >= self.width() || c.y >= self.height() {
            false
        } else {
            self.el[c.y as usize * self.width + c.x as usize] = v;
            true
        }
    }

    pub fn height(&self) -> i64 {
        self.height as i64
    }

    pub fn width(&self) -> i64 {
        self.width as i64
    }

    pub fn col(&self, x: i64) -> Option<Vec<&T>> {
        (0..self.height)
            .map(|y| self.at(&Coords { x, y: y as i64 }))
            .collect()
    }

    pub fn row(&self, y: i64) -> Option<Vec<&T>> {
        (0..self.width)
            .map(|x| self.at(&Coords { x: x as i64, y }))
            .collect()
    }

    pub fn neighbors(&self, c: &Coords) -> Vec<Option<&T>> {
        vec![
            self.at(&Coords { x: c.x, y: c.y - 1 }),
            self.at(&Coords {
                x: c.x + 1,
                y: c.y - 1,
            }),
            self.at(&Coords { x: c.x + 1, y: c.y }),
            self.at(&Coords {
                x: c.x + 1,
                y: c.y + 1,
            }),
            self.at(&Coords { x: c.x, y: c.y + 1 }),
            self.at(&Coords {
                x: c.x - 1,
                y: c.y + 1,
            }),
            self.at(&Coords { x: c.x - 1, y: c.y }),
            self.at(&Coords {
                x: c.x - 1,
                y: c.y - 1,
            }),
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
    pub fn traverse(&self, d: &Coords) -> TraverseIter<T> {
        TraverseIter::new(&self, &Coords { x: 0, y: 0 }, d, Wrap::None)
    }

    pub fn traverse_wrap(&self, d: &Coords, wrap: Wrap) -> TraverseIter<T> {
        TraverseIter::new(&self, &Coords { x: 0, y: 0 }, d, wrap)
    }

    pub fn traverse_init_wrap(&self, init: &Coords, d: &Coords, wrap: Wrap) -> TraverseIter<T> {
        TraverseIter::new(&self, init, d, wrap)
    }
}

impl<T: std::cmp::PartialEq> Grid2D<T> {
    pub fn count(&self, v: T) -> usize {
        self.iter().filter(|&x| x == &v).count()
    }
}

impl<T: Copy> Grid2D<T> {
    pub fn rotate90(&self) -> Grid2D<T> {
        let mut new_el = self.el.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                new_el[y * self.width + x] = self.el[(self.height - 1 - x) * self.width + y];
            }
        }
        Grid2D {
            el: new_el,
            width: self.height,
            height: self.width,
        }
    }

    pub fn flip(&self, flip_param: Flip) -> Grid2D<T> {
        let mut new_el = self.el.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                match flip_param {
                    Flip::FlipNone => {}
                    Flip::FlipH => {
                        new_el[y * self.width + x] = self.el[y * self.width + (self.width - 1 - x)];
                    }
                    Flip::FlipV => {
                        new_el[y * self.width + x] =
                            self.el[(self.height - 1 - y) * self.width + x];
                    }
                }
            }
        }
        Grid2D {
            el: new_el,
            width: self.height,
            height: self.width,
        }
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
    cur: Coords,
}

impl<'a, T> Iter<'a, T> {
    fn new(grid: &'a Grid2D<T>) -> Iter<T> {
        Iter {
            grid,
            cur: Coords { x: 0, y: 0 },
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let ret = self.grid.at(&self.cur);
        if self.cur.x + 1 < self.grid.width() {
            self.cur.x += 1;
        } else {
            self.cur.x = 0;
            self.cur.y += 1;
        }
        ret
    }
}

pub struct CoordsIter<'a, T> {
    grid: &'a Grid2D<T>,
    cur: Coords,
}

impl<'a, T> CoordsIter<'a, T> {
    fn new(grid: &'a Grid2D<T>) -> CoordsIter<T> {
        CoordsIter {
            grid,
            cur: Coords { x: 0, y: 0 },
        }
    }
}

impl<'a, T> Iterator for CoordsIter<'a, T> {
    type Item = Coords;

    fn next(&mut self) -> Option<Coords> {
        let ret = if self.cur.x < 0
            || self.cur.y < 0
            || self.cur.x >= self.grid.width()
            || self.cur.y >= self.grid.height()
        {
            None
        } else {
            Some(self.cur.clone())
        };
        if self.cur.x + 1 < self.grid.width() {
            self.cur.x += 1;
        } else {
            self.cur.x = 0;
            self.cur.y += 1;
        }
        ret
    }
}

pub struct TraverseIter<'a, T> {
    grid: &'a Grid2D<T>,
    cur: Coords,
    d: Coords,
    wrap: Wrap,
}

impl<'a, T> TraverseIter<'a, T> {
    fn new(grid: &'a Grid2D<T>, init: &Coords, d: &Coords, wrap: Wrap) -> TraverseIter<'a, T> {
        TraverseIter {
            grid,
            cur: init.clone(),
            d: d.clone(),
            wrap,
        }
    }
}

impl<'a, T> Iterator for TraverseIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let ret = self.grid.at(&self.cur);
        match self.wrap {
            Wrap::None => {
                self.cur.x += self.d.x;
                self.cur.y += self.d.y;
            }
            Wrap::WrapX => {
                self.cur.x = (self.cur.x + self.d.x) % self.grid.width();
                self.cur.y += self.d.y;
            }
            Wrap::WrapY => {
                self.cur.x += self.d.x;
                self.cur.y = (self.cur.y + self.d.y) % self.grid.height();
            }
            Wrap::WrapXY => {
                self.cur.x = (self.cur.x + self.d.x) % self.grid.width();
                self.cur.y = (self.cur.y + self.d.y) % self.grid.height();
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let example = Grid2D::new(
            r#"abc
def
ghi"#,
        )
        .unwrap();
        let rot90 = example.rotate90();
        assert_eq!(
            format!("{}", rot90),
            r#"gda
heb
ifc"#
        );

        let rot180 = rot90.rotate90();
        let rot270 = rot180.rotate90();
        let rot360 = rot270.rotate90();
        assert_eq!(format!("{}", example), format!("{}", rot360));
    }

    #[test]
    fn test_flip() {
        let example = Grid2D::new(
            r#"abc
def
ghi"#,
        )
        .unwrap();

        let fliph = example.flip(Flip::FlipH);
        assert_eq!(
            format!("{}", fliph),
            r#"cba
fed
ihg"#
        );
        let fliph2 = fliph.flip(Flip::FlipH);
        assert_eq!(format!("{}", example), format!("{}", fliph2));

        let flipv = example.flip(Flip::FlipV);
        assert_eq!(
            format!("{}", flipv),
            r#"ghi
def
abc"#
        );
        let flipv2 = flipv.flip(Flip::FlipV);
        assert_eq!(format!("{}", example), format!("{}", flipv2));
    }

    #[test]
    fn test_row() {
        let example = Grid2D::new(
            r#"abc
def
ghi"#,
        )
        .unwrap();
        let s: String = example.row(0).unwrap().into_iter().collect();
        assert_eq!(s, "abc");
        let s: String = example.row(2).unwrap().into_iter().collect();
        assert_eq!(s, "ghi");
    }

    #[test]
    fn test_col() {
        let example = Grid2D::new(
            r#"abc
def
ghi"#,
        )
        .unwrap();
        let s: String = example.col(0).unwrap().into_iter().collect();
        assert_eq!(s, "adg");
        let s: String = example.col(2).unwrap().into_iter().collect();
        assert_eq!(s, "cfi");
    }
}
