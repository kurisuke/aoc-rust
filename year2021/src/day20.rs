use common::day::Day;
use util::grid2d::{Coords, Grid2D};

pub struct Day20 {}

fn parse_input(input: &str) -> (Vec<bool>, Grid2D<bool>) {
    let p1 = input.split("\n\n").next().unwrap();
    let algo = p1
        .chars()
        .map(|c| match c {
            '#' => true,
            '.' => false,
            _ => unreachable!(),
        })
        .collect();

    let p2 = input.split("\n\n").nth(1).unwrap();
    let input = Grid2D::new_by(p2, |c| match c {
        '#' => true,
        '.' => false,
        _ => unreachable!(),
    })
    .unwrap();
    (algo, input)
}

#[rustfmt::skip]
fn pixel_value(c: &Coords, algo: &[bool], input: &Grid2D<bool>, def: bool) -> bool {
    let addr = ((*input.at(&Coords {x: c.x - 1, y: c.y - 1}).unwrap_or(&def) as usize) << 8) +
        ((*input.at(&Coords {x: c.x, y: c.y - 1}).unwrap_or(&def) as usize) << 7) +
        ((*input.at(&Coords {x: c.x + 1, y: c.y - 1}).unwrap_or(&def) as usize) << 6) +
        ((*input.at(&Coords {x: c.x - 1, y: c.y}).unwrap_or(&def) as usize) << 5) +
        ((*input.at(&Coords {x: c.x, y: c.y}).unwrap_or(&def) as usize) << 4) +
        ((*input.at(&Coords {x: c.x + 1, y: c.y}).unwrap_or(&def) as usize) << 3) +
        ((*input.at(&Coords {x: c.x - 1, y: c.y + 1}).unwrap_or(&def) as usize) << 2) +
        ((*input.at(&Coords {x: c.x, y: c.y + 1}).unwrap_or(&def) as usize) << 1) +
        *input.at(&Coords {x: c.x + 1, y: c.y + 1}).unwrap_or(&def) as usize;
    algo[addr]
}

fn enhance(algo: &[bool], input: &Grid2D<bool>, def: bool) -> Grid2D<bool> {
    let output_dims = Coords {
        x: input.width() + 2,
        y: input.width() + 2,
    };
    let mut output = Grid2D::with_default(output_dims, &false);
    let oc: Vec<_> = output.coords_iter().collect();
    for c in oc.iter() {
        let c_offset = Coords {
            x: c.x - 1,
            y: c.y - 1,
        };
        output.set(c, pixel_value(&c_offset, algo, input, def));
    }
    output
}

fn enhance_twice(algo: &[bool], image: &Grid2D<bool>) -> Grid2D<bool> {
    let (def_odd, def_even) = if algo[0] && !algo[511] {
        (false, true)
    } else if !algo[0] {
        (false, false)
    } else {
        unreachable!() // no implementation
    };
    let image = enhance(algo, image, def_odd);
    enhance(algo, &image, def_even)
}

impl Day for Day20 {
    fn star1(&self, input: &str) -> String {
        let (algo, image) = parse_input(input);
        let image = enhance_twice(&algo, &image);
        let num_set = image.iter().filter(|v| **v).count();
        format!("{}", num_set)
    }

    fn star2(&self, input: &str) -> String {
        let (algo, mut image) = parse_input(input);
        for _ in 0..25 {
            image = enhance_twice(&algo, &image);
        }
        let num_set = image.iter().filter(|v| **v).count();
        format!("{}", num_set)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"#;

        let d = Day20 {};
        assert_eq!(d.star1(input), "35");
        assert_eq!(d.star2(input), "3351");
    }
}
