use itertools::concat;
use std::cmp::Ordering;
use std::fmt::{self, Display, Formatter};
use std::iter::Zip;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Eq, PartialEq, Hash)]
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

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Flip {
    FlipNone,
    FlipH,
    FlipV,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::N => Direction::S,
            Direction::NE => Direction::SW,
            Direction::E => Direction::W,
            Direction::SE => Direction::NW,
            Direction::S => Direction::N,
            Direction::SW => Direction::NE,
            Direction::W => Direction::E,
            Direction::NW => Direction::SE,
        }
    }

    pub fn left45(&self) -> Direction {
        match self {
            Direction::N => Direction::NW,
            Direction::NE => Direction::N,
            Direction::E => Direction::NE,
            Direction::SE => Direction::E,
            Direction::S => Direction::SE,
            Direction::SW => Direction::S,
            Direction::W => Direction::SW,
            Direction::NW => Direction::W,
        }
    }

    pub fn right45(&self) -> Direction {
        match self {
            Direction::N => Direction::NE,
            Direction::NE => Direction::E,
            Direction::E => Direction::SE,
            Direction::SE => Direction::S,
            Direction::S => Direction::SW,
            Direction::SW => Direction::W,
            Direction::W => Direction::NW,
            Direction::NW => Direction::N,
        }
    }

    pub fn left90(&self) -> Direction {
        match self {
            Direction::N => Direction::W,
            Direction::NE => Direction::NW,
            Direction::E => Direction::N,
            Direction::SE => Direction::NE,
            Direction::S => Direction::E,
            Direction::SW => Direction::SE,
            Direction::W => Direction::S,
            Direction::NW => Direction::SW,
        }
    }

    pub fn right90(&self) -> Direction {
        match self {
            Direction::N => Direction::E,
            Direction::NE => Direction::SE,
            Direction::E => Direction::S,
            Direction::SE => Direction::SW,
            Direction::S => Direction::W,
            Direction::SW => Direction::NW,
            Direction::W => Direction::N,
            Direction::NW => Direction::NE,
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Coords {
    pub x: i64,
    pub y: i64,
}

impl fmt::Display for Coords {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl Ord for Coords {
    fn cmp(&self, other: &Self) -> Ordering {
        self.y.cmp(&other.y).then_with(|| self.x.cmp(&other.x))
    }
}

impl PartialOrd for Coords {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<(i64, i64)> for Coords {
    fn from(p: (i64, i64)) -> Self {
        Coords { x: p.0, y: p.1 }
    }
}

impl Add for Coords {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Coords {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Sub for Coords {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Coords {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl Coords {
    pub fn manhattan(&self, other: &Coords) -> u64 {
        (self.x - other.x).unsigned_abs() + (self.y - other.y).unsigned_abs()
    }

    pub fn mov(&self, d: Direction) -> Coords {
        match d {
            Direction::N => Coords {
                x: self.x,
                y: self.y - 1,
            },
            Direction::NE => Coords {
                x: self.x + 1,
                y: self.y - 1,
            },
            Direction::E => Coords {
                x: self.x + 1,
                y: self.y,
            },
            Direction::SE => Coords {
                x: self.x + 1,
                y: self.y + 1,
            },
            Direction::S => Coords {
                x: self.x,
                y: self.y + 1,
            },
            Direction::SW => Coords {
                x: self.x - 1,
                y: self.y + 1,
            },
            Direction::W => Coords {
                x: self.x - 1,
                y: self.y,
            },
            Direction::NW => Coords {
                x: self.x - 1,
                y: self.y - 1,
            },
        }
    }
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
    pub fn new_by<F>(input: &str, convert: F) -> Result<Grid2D<T>, Grid2DError>
    where
        F: Fn(char) -> T,
    {
        let grid_ch = Grid2D::new(input)?;
        let el: Vec<T> = grid_ch.iter().map(|c| convert(*c)).collect();
        Ok(Grid2D {
            el,
            width: grid_ch.width() as usize,
            height: grid_ch.height() as usize,
        })
    }

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

    pub fn dimensions(&self) -> Coords {
        Coords {
            x: self.width as i64,
            y: self.height as i64,
        }
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

    pub fn set_col(&mut self, x: i64, vals: Vec<T>) -> bool {
        if x < 0 || x >= self.width() || vals.len() != self.height {
            false
        } else {
            for (y, val) in vals.into_iter().enumerate() {
                self.set(&Coords { x, y: y as i64 }, val);
            }
            true
        }
    }

    pub fn set_row(&mut self, y: i64, vals: Vec<T>) -> bool {
        if y < 0 || y >= self.height() || vals.len() != self.width {
            false
        } else {
            for (x, val) in vals.into_iter().enumerate() {
                self.set(&Coords { x: x as i64, y }, val);
            }
            true
        }
    }

    #[allow(dead_code)]
    pub fn neighbors_cardinal(&self, c: &Coords) -> Vec<Option<&T>> {
        vec![
            self.at(&Coords { x: c.x, y: c.y - 1 }),
            self.at(&Coords { x: c.x + 1, y: c.y }),
            self.at(&Coords { x: c.x, y: c.y + 1 }),
            self.at(&Coords { x: c.x - 1, y: c.y }),
        ]
    }

    pub fn neighbors_cardinal_coords(&self, c: &Coords) -> Vec<Coords> {
        vec![
            Coords { x: c.x, y: c.y - 1 },
            Coords { x: c.x + 1, y: c.y },
            Coords { x: c.x, y: c.y + 1 },
            Coords { x: c.x - 1, y: c.y },
        ]
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

    #[allow(dead_code)]
    pub fn neighbors_coords(&self, c: &Coords) -> Vec<Coords> {
        vec![
            Coords { x: c.x, y: c.y - 1 },
            Coords {
                x: c.x + 1,
                y: c.y - 1,
            },
            Coords { x: c.x + 1, y: c.y },
            Coords {
                x: c.x + 1,
                y: c.y + 1,
            },
            Coords { x: c.x, y: c.y + 1 },
            Coords {
                x: c.x - 1,
                y: c.y + 1,
            },
            Coords { x: c.x - 1, y: c.y },
            Coords {
                x: c.x - 1,
                y: c.y - 1,
            },
        ]
    }

    pub fn iter(&self) -> Iter<T> {
        Iter::new(self)
    }

    #[allow(dead_code)]
    pub fn coords_iter(&self) -> CoordsIter<T> {
        CoordsIter::new(self)
    }

    pub fn enumerate(&self) -> Zip<CoordsIter<T>, Iter<T>> {
        CoordsIter::new(self).zip(Iter::new(self))
    }

    #[allow(dead_code)]
    pub fn traverse(&self, d: &Coords) -> TraverseIter<T> {
        TraverseIter::new(self, &Coords { x: 0, y: 0 }, d, Wrap::None)
    }

    pub fn traverse_wrap(&self, d: &Coords, wrap: Wrap) -> TraverseIter<T> {
        TraverseIter::new(self, &Coords { x: 0, y: 0 }, d, wrap)
    }

    pub fn traverse_init_wrap(&self, init: &Coords, d: &Coords, wrap: Wrap) -> TraverseIter<T> {
        TraverseIter::new(self, init, d, wrap)
    }
}

impl<T: std::cmp::PartialEq> Grid2D<T> {
    pub fn count(&self, v: T) -> usize {
        self.iter().filter(|&x| x == &v).count()
    }

    pub fn find(&self, v: T) -> Option<Coords> {
        self.enumerate().find(|&(_, x)| x == &v).map(|(c, _)| c)
    }

    pub fn filter(&self, vs: &[T]) -> Vec<Coords> {
        self.enumerate()
            .filter(|&(_, x)| vs.contains(x))
            .map(|(c, _)| c)
            .collect()
    }
}

impl<T: Copy + Default> Grid2D<T> {
    pub fn with_default(dims: Coords, def: &T) -> Grid2D<T> {
        let el = vec![def; dims.x as usize * dims.y as usize];
        Grid2D {
            el: el.into_iter().cloned().collect(),
            width: dims.x as usize,
            height: dims.y as usize,
        }
    }

    pub fn transpose(&self) -> Grid2D<T> {
        let mut grid_new = Grid2D::with_default(
            Coords {
                x: self.height(),
                y: self.width(),
            },
            &T::default(),
        );

        for c in self.coords_iter() {
            grid_new.set(&Coords { x: c.y, y: c.x }, *self.at(&c).unwrap());
        }

        grid_new
    }

    pub fn clip(&self, c1: Coords, c2: Coords) -> Option<Grid2D<T>> {
        if c1.x < 0
            || c1.y < 0
            || c2.x < c1.x
            || c2.y < c1.y
            || c2.x > self.width()
            || c2.y > self.height()
        {
            None
        } else {
            let width = (c2.x - c1.x) as usize;
            let height = (c2.y - c1.y) as usize;
            let mut el = vec![];
            for y in c1.y..c2.y {
                for x in c1.x..c2.x {
                    el.push(self.el[y as usize * self.width + x as usize]);
                }
            }
            Some(Grid2D { el, width, height })
        }
    }

    pub fn paste(&mut self, c: Coords, other: &Grid2D<T>) -> bool {
        if c.x < 0
            || c.y < 0
            || c.x + other.width() > self.width()
            || c.y + other.width() > self.height()
        {
            false
        } else {
            for (co, v) in other.enumerate() {
                self.el[(c.y + co.y) as usize * self.width + (c.x + co.x) as usize] = *v;
            }
            true
        }
    }

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
        if flip_param == Flip::FlipNone {
            return self.clone();
        }

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

    pub fn transformations(&self) -> Vec<Grid2D<T>> {
        let mut trans = vec![];
        let mut orig = self.clone();
        let mut flipped = self.flip(Flip::FlipH);
        for _ in 0..4 {
            trans.push(orig.clone());
            trans.push(flipped.clone());
            orig = orig.rotate90();
            flipped = flipped.rotate90();
        }
        trans
    }
}

impl<T: std::cmp::PartialEq + Copy> Grid2D<T> {
    pub fn replace(&mut self, from: &T, to: &T) {
        for c in self.filter(&[*from]) {
            self.set(&c, *to);
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

impl<T> Iter<'_, T> {
    fn new(grid: &Grid2D<T>) -> Iter<T> {
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

impl<T> CoordsIter<'_, T> {
    fn new(grid: &Grid2D<T>) -> CoordsIter<T> {
        CoordsIter {
            grid,
            cur: Coords { x: 0, y: 0 },
        }
    }
}

impl<T> Iterator for CoordsIter<'_, T> {
    type Item = Coords;

    fn next(&mut self) -> Option<Coords> {
        let ret = if self.cur.x < 0
            || self.cur.y < 0
            || self.cur.x >= self.grid.width()
            || self.cur.y >= self.grid.height()
        {
            None
        } else {
            Some(self.cur)
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
            cur: *init,
            d: *d,
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

    #[test]
    fn test_clip() {
        let example = Grid2D::new(
            r#"abcd
efgh
ijkl
mnop"#,
        )
        .unwrap();
        let clip = example
            .clip(Coords { x: 1, y: 1 }, Coords { x: 3, y: 3 })
            .unwrap();
        assert_eq!(format!("{}", clip), "fg\njk");
    }

    #[test]
    fn test_paste() {
        let mut example = Grid2D::new(
            r#"abcd
efgh
ijkl
mnop"#,
        )
        .unwrap();

        let other = Grid2D::new("rs\ntu").unwrap();
        assert!(example.paste(Coords { x: 1, y: 1 }, &other));
        assert_eq!(
            format!("{}", example),
            r#"abcd
ersh
itul
mnop"#
        );
    }
}
