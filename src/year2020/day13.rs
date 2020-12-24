use crate::day::Day;

pub struct Day13 {}

struct InputInfo {
    ts: u64,
    bus_infos: Vec<BusInfo>,
}

struct BusInfo {
    id: u64,
    dep_diff: u64,
}

impl Day for Day13 {
    fn star1(&self, input: &str) -> String {
        let input_info = parse_input(&input);
        let ts = input_info.ts;
        let first_bus = input_info
            .bus_infos
            .into_iter()
            .map(|x| (x.id, x.id - ts % x.id))
            .fold((0, ts), |min, x| if x.1 < min.1 { x } else { min });

        format!("{}", first_bus.0 * first_bus.1)
    }

    fn star2(&self, input: &str) -> String {
        let input_info = parse_input(&input);

        let mut mul = input_info.bus_infos[0].id;
        let mut off = input_info.bus_infos[0].dep_diff;

        for bus in input_info.bus_infos.iter().skip(1) {
            for x in 0..bus.id {
                if (off + x * mul + bus.dep_diff) % bus.id == 0 {
                    off += x * mul;
                    mul *= bus.id;
                    break;
                }
            }
        }

        format!("{}", off)
    }
}

fn parse_input(input: &str) -> InputInfo {
    let mut it = input.lines();
    let ts = it.next().unwrap().parse::<u64>().unwrap();
    let bus_infos: Vec<_> = it
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|&x| x.1 != "x")
        .map(|x| BusInfo {
            id: x.1.parse::<u64>().unwrap(),
            dep_diff: x.0 as u64,
        })
        .collect();
    InputInfo { ts, bus_infos }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"939
7,13,x,x,59,x,31,19"#;
        let d = Day13 {};
        assert_eq!(d.star1(input), "295");
        assert_eq!(d.star2(input), "1068781");
    }

    #[test]
    fn ex2() {
        let input = r#"0
67,7,59,61"#;
        let d = Day13 {};
        assert_eq!(d.star2(input), "754018");
    }

    #[test]
    fn ex3() {
        let input = r#"0
67,x,7,59,61"#;
        let d = Day13 {};
        assert_eq!(d.star2(input), "779210");
    }

    #[test]
    fn ex4() {
        let input = r#"0
67,7,x,59,61"#;
        let d = Day13 {};
        assert_eq!(d.star2(input), "1261476");
    }

    #[test]
    fn ex5() {
        let input = r#"0
1789,37,47,1889"#;
        let d = Day13 {};
        assert_eq!(d.star2(input), "1202161486");
    }
}
