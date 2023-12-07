use std::collections::HashMap;

use common::day::Day;

pub struct Day05 {}

impl Day for Day05 {
    fn star1(&self, input: &str) -> String {
        let input = Input::parse(input);
        input.lowest_location().to_string()
    }

    fn star2(&self, _input: &str) -> String {
        String::from("not implemented")
    }
}

struct Input {
    seeds: Vec<usize>,
    maps: Maps,
}

impl Input {
    fn parse(input: &str) -> Input {
        let mut sections = input.split("\n\n");

        let seeds = sections.next().unwrap();
        let seeds = seeds
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect();

        let maps = sections
            .map(Map::parse)
            .map(|map| (map.src.clone(), map))
            .collect();

        Input { seeds, maps }
    }

    fn convert(&self, seed: usize, start: &str, end: &str) -> usize {
        let mut cur_src = start;
        let mut cur = seed;
        while cur_src != end {
            let cur_map = self.maps.get(cur_src).unwrap();

            for map_line in &cur_map.lines {
                if cur >= map_line.src && cur < map_line.src + map_line.len {
                    cur = map_line.dest + (cur - map_line.src);
                    break;
                }
            }

            cur_src = &cur_map.dest;
        }
        cur
    }

    fn lowest_location(&self) -> usize {
        self.seeds
            .iter()
            .map(|seed| self.convert(*seed, "seed", "location"))
            .min()
            .unwrap()
    }
}

type Maps = HashMap<String, Map>;

struct Map {
    src: String,
    dest: String,
    lines: Vec<MapLine>,
}

impl Map {
    fn parse(sec: &str) -> Map {
        let mut lines = sec.lines();
        let heading = lines.next().unwrap().split_whitespace().next().unwrap();
        let src = heading.split('-').next().unwrap().to_string();
        let dest = heading.split('-').nth(2).unwrap().to_string();
        let lines = lines.map(MapLine::parse).collect();
        Map { src, dest, lines }
    }
}

struct MapLine {
    dest: usize,
    src: usize,
    len: usize,
}

impl MapLine {
    fn parse(line: &str) -> MapLine {
        let mut spl = line.split_whitespace();
        MapLine {
            dest: spl.next().unwrap().parse().unwrap(),
            src: spl.next().unwrap().parse().unwrap(),
            len: spl.next().unwrap().parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn ex1() {
        let d = Day05 {};
        assert_eq!(d.star1(INPUT), "35");
    }
}
