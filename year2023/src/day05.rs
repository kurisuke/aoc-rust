use common::day::Day;

pub struct Day05 {}

impl Day for Day05 {
    fn star1(&self, input: &str) -> String {
        let input = Input::parse(input);
        input.lowest_location().to_string()
    }

    fn star2(&self, input: &str) -> String {
        let input = Input::parse(input);
        let mut location = 0;
        loop {
            let seed = input.convert_rev(location);
            if input.in_seed_range(seed) {
                break;
            }
            location += 1;
        }
        location.to_string()
    }
}

struct Input {
    seeds: Vec<usize>,
    maps: Vec<Map>,
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

        let maps = sections.map(Map::parse).collect();

        Input { seeds, maps }
    }

    fn convert(&self, seed: usize) -> usize {
        let mut cur = seed;
        for cur_map in &self.maps {
            for map_line in &cur_map.lines {
                if cur >= map_line.src && cur < map_line.src + map_line.len {
                    cur = map_line.dest + (cur - map_line.src);
                    break;
                }
            }
        }
        cur
    }

    fn convert_rev(&self, seed: usize) -> usize {
        let mut cur = seed;

        for cur_map in self.maps.iter().rev() {
            for map_line in &cur_map.lines {
                if cur >= map_line.dest && cur < map_line.dest + map_line.len {
                    cur = map_line.src + (cur - map_line.dest);
                    break;
                }
            }
        }
        cur
    }

    fn in_seed_range(&self, n: usize) -> bool {
        for c in self.seeds.chunks(2) {
            if n >= c[0] && n < c[0] + c[1] {
                return true;
            }
        }
        false
    }

    fn lowest_location(&self) -> usize {
        self.seeds
            .iter()
            .map(|seed| self.convert(*seed))
            .min()
            .unwrap()
    }
}

struct Map {
    lines: Vec<MapLine>,
}

impl Map {
    fn parse(sec: &str) -> Map {
        let lines = sec.lines().skip(1).map(MapLine::parse).collect();
        Map { lines }
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
    fn star1() {
        let d = Day05 {};
        assert_eq!(d.star1(INPUT), "35");
    }

    #[test]
    fn convert_rev() {
        let input = Input::parse(INPUT);
        assert_eq!(input.convert_rev(82), 79);
        assert_eq!(input.convert_rev(43), 14);
        assert_eq!(input.convert_rev(86), 55);
        assert_eq!(input.convert_rev(35), 13);
    }

    #[test]
    fn star2() {
        let d = Day05 {};
        assert_eq!(d.star2(INPUT), "46");
    }
}
