use common::day::Day;
use std::collections::HashSet;
use util::grid2d::{Coords, Grid2D};

pub struct Day10 {}

fn parse_input(input: &str) -> Grid2D<bool> {
    Grid2D::new_by(input, |c| match c {
        '#' => true,
        '.' => false,
        _ => unreachable!(),
    })
    .unwrap()
}

fn best_pos(grid: &Grid2D<bool>) -> (Coords, usize) {
    grid.coords_iter()
        .filter(|pos| *grid.at(pos).unwrap())
        .map(|pos| {
            (
                pos,
                all_vecs(&grid.dimensions(), &pos)
                    .iter()
                    .filter(|v| hit(grid, &pos, v).is_some())
                    .count(),
            )
        })
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
}

fn all_vecs(dimensions: &Coords, pos: &Coords) -> HashSet<Coords> {
    let mut vecs = HashSet::new();
    for x in 0..dimensions.x {
        for y in 0..dimensions.y {
            let v = Coords {
                x: x - pos.x,
                y: y - pos.y,
            };
            if v.x != 0 || v.y != 0 {
                vecs.insert(reduced_vec(&v));
            }
        }
    }
    vecs
}

fn reduced_vec(v: &Coords) -> Coords {
    let g = gcd(v.x.abs(), v.y.abs());
    Coords {
        x: v.x / g,
        y: v.y / g,
    }
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let tmp = a % b;
        a = b;
        b = tmp;
    }
    a
}

fn hit(grid: &Grid2D<bool>, pos: &Coords, v: &Coords) -> Option<Coords> {
    let mut p_hit = *pos + *v;
    while let Some(x) = grid.at(&p_hit) {
        if *x {
            return Some(p_hit);
        } else {
            p_hit += *v;
        }
    }
    None
}

fn vec_dir(v: &Coords) -> f64 {
    let angle = (v.y as f64).atan2(v.x as f64);
    let degrees = 180f64 * angle / std::f64::consts::PI;
    (degrees + 450f64) % 360f64
}

fn zap(grid: &mut Grid2D<bool>, pos: &Coords, n: usize) -> Coords {
    let mut ordered_vecs: Vec<_> = all_vecs(&grid.dimensions(), pos).into_iter().collect();
    ordered_vecs.sort_by(|a, b| vec_dir(a).partial_cmp(&vec_dir(b)).unwrap());

    let mut zapped = 0;
    let mut i = 0;
    loop {
        let vec = ordered_vecs[i % ordered_vecs.len()];
        if let Some(hit_pos) = hit(grid, pos, &vec) {
            zapped += 1;
            if zapped == n {
                return hit_pos;
            }

            grid.set(&hit_pos, false);
        }
        i += 1;
    }
}

impl Day for Day10 {
    fn star1(&self, input: &str) -> String {
        let grid = parse_input(input);
        format!("{}", best_pos(&grid).1)
    }

    fn star2(&self, input: &str) -> String {
        let mut grid = parse_input(input);
        let base_pos = best_pos(&grid).0;
        let hit_200 = zap(&mut grid, &base_pos, 200);
        format!("{}", hit_200.x * 100 + hit_200.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"#;

        let d = Day10 {};
        assert_eq!(d.star1(input), "210");
        assert_eq!(d.star2(input), "802");
    }
}
