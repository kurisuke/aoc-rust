use common::day::Day;
use util::grid2d::{Coords, Grid2D, Wrap};

pub struct Day08 {}

fn visible(grid: &Grid2D<u32>, c: &Coords) -> bool {
    let this_height = grid.at(c).unwrap();
    for mut dir_iter in [
        grid.traverse_init_wrap(c, &Coords::from((-1, 0)), Wrap::None)
            .skip(1),
        grid.traverse_init_wrap(c, &Coords::from((1, 0)), Wrap::None)
            .skip(1),
        grid.traverse_init_wrap(c, &Coords::from((0, -1)), Wrap::None)
            .skip(1),
        grid.traverse_init_wrap(c, &Coords::from((0, 1)), Wrap::None)
            .skip(1),
    ] {
        let visible_from_direction = dir_iter.all(|other_height| other_height < this_height);
        if visible_from_direction {
            return true;
        }
    }

    false
}

fn scenic_score(grid: &Grid2D<u32>, c: &Coords) -> usize {
    let this_height = grid.at(c).unwrap();

    let dir_iters = [
        grid.traverse_init_wrap(c, &Coords::from((-1, 0)), Wrap::None)
            .skip(1),
        grid.traverse_init_wrap(c, &Coords::from((1, 0)), Wrap::None)
            .skip(1),
        grid.traverse_init_wrap(c, &Coords::from((0, -1)), Wrap::None)
            .skip(1),
        grid.traverse_init_wrap(c, &Coords::from((0, 1)), Wrap::None)
            .skip(1),
    ];
    dir_iters
        .into_iter()
        .map(|it| {
            let mut view_distance = 0;
            for other_height in it {
                view_distance += 1;
                if other_height >= this_height {
                    break;
                }
            }
            view_distance
        })
        .product()
}

fn find_num_visible(grid: &Grid2D<u32>) -> usize {
    let dims = grid.dimensions();
    let num_outside = (2 * dims.x + 2 * dims.y - 4) as usize;

    let mut num_visible = num_outside;
    for y in 1..(dims.y - 1) {
        for x in 1..(dims.x - 1) {
            if visible(grid, &Coords::from((x, y))) {
                num_visible += 1;
            }
        }
    }

    num_visible
}

fn find_max_scenic_score(grid: &Grid2D<u32>) -> usize {
    let dims = grid.dimensions();

    let mut max_score = 1;
    for y in 1..(dims.y - 1) {
        for x in 1..(dims.x - 1) {
            max_score = max_score.max(scenic_score(grid, &Coords::from((x, y))));
        }
    }

    max_score
}

impl Day for Day08 {
    fn star1(&self, input: &str) -> String {
        let grid = Grid2D::new_by(input, |c| c.to_digit(10).unwrap()).unwrap();
        format!("{}", find_num_visible(&grid))
    }

    fn star2(&self, input: &str) -> String {
        let grid = Grid2D::new_by(input, |c| c.to_digit(10).unwrap()).unwrap();
        format!("{}", find_max_scenic_score(&grid))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"30373
25512
65332
33549
35390"#;

        let d = Day08 {};
        assert_eq!(d.star1(input), "21");
        assert_eq!(d.star2(input), "8");
    }
}
