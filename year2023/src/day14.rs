use common::day::Day;
use util::grid2d::Grid2D;

pub struct Day14 {}

impl Day for Day14 {
    fn star1(&self, input: &str) -> String {
        let mut grid = Grid2D::new(input).unwrap();
        slide_north(&mut grid);
        load(&grid).to_string()
    }

    fn star2(&self, _input: &str) -> String {
        String::from("not implemented")
    }
}

fn slide_north(grid: &mut Grid2D<char>) {
    for i in 0..grid.width() {
        grid.set_col(i, slide_col_north(grid.col(i).unwrap()));
    }
}

fn slide_col_north(col: Vec<&char>) -> Vec<char> {
    let mut col_new = vec![];
    for range in col.split(|x| x == &&'#') {
        let num_movable = range.into_iter().filter(|x| x == &&&'O').count();
        col_new.extend(std::iter::repeat('O').take(num_movable));
        col_new.extend(std::iter::repeat('.').take(range.len() - num_movable));
        col_new.push('#');
    }
    col_new.pop();
    col_new
}

fn load(grid: &Grid2D<char>) -> usize {
    (0..grid.height())
        .map(|i| {
            grid.row(i)
                .unwrap()
                .into_iter()
                .filter(|x| x == &&'O')
                .count()
                * (grid.height() - i) as usize
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

    #[test]
    fn ex1() {
        let d = Day14 {};
        assert_eq!(d.star1(INPUT), "136");
    }
}
