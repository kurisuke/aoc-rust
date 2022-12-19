use common::day::Day;

pub struct Day19 {}

struct Blueprint {
    id: usize,
    ore_robot_cost: usize,
    clay_robot_cost: usize,
    obsidian_robot_cost: (usize, usize),
    geode_robot_cost: (usize, usize),
}

impl Blueprint {
    fn parse(line: &str, id: usize) -> Blueprint {
        let tokens: Vec<_> = line.split_whitespace().collect();
        Blueprint {
            id,
            ore_robot_cost: tokens[6].parse().unwrap(),
            clay_robot_cost: tokens[12].parse().unwrap(),
            obsidian_robot_cost: (tokens[18].parse().unwrap(), tokens[21].parse().unwrap()),
            geode_robot_cost: (tokens[27].parse().unwrap(), tokens[30].parse().unwrap()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(usize)]
enum Resource {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}

type Stash = [usize; 4];
type Robots = [usize; 4];

#[derive(Copy, Clone)]
struct State {
    minutes: usize,
    stash: Stash,
    robots: Robots,
}

impl State {
    fn new(minutes: usize) -> State {
        State {
            minutes,
            stash: [0; 4],
            robots: [1, 0, 0, 0],
        }
    }
}

fn geodes(blueprint: &Blueprint, minutes: usize) -> usize {
    let mut best_so_far = vec![0; minutes];
    let state = State::new(minutes);
    search(blueprint, state, &mut best_so_far)
}

fn search(blueprint: &Blueprint, state: State, best_so_far: &mut [usize]) -> usize {
    if state.minutes == 0 || state.robots[Resource::Geode as usize] < best_so_far[state.minutes - 1]
    {
        state.stash[Resource::Geode as usize]
    } else {
        best_so_far[state.minutes - 1] = state.robots[Resource::Geode as usize];
        let mut max_geodes = 0;
        for robot_type in choose_build(blueprint, &state) {
            let mut new_state = state;
            if wait_for_next_robot(blueprint, &mut new_state, robot_type) {
                new_state.robots[robot_type as usize] += 1;
                max_geodes = max_geodes.max(search(blueprint, new_state, best_so_far));
            }
        }
        max_geodes
    }
}

fn choose_build(blueprint: &Blueprint, state: &State) -> Vec<Resource> {
    let mut builds = vec![Resource::Geode];

    // if we get enough resources per round to build any robot requiring that resource,
    // we don't need to increase our production, thus no need to build more robots of
    // this type
    if state.robots[Resource::Ore as usize] < blueprint.ore_robot_cost
        || state.robots[Resource::Ore as usize] < blueprint.clay_robot_cost
        || state.robots[Resource::Ore as usize] < blueprint.obsidian_robot_cost.0
        || state.robots[Resource::Ore as usize] < blueprint.geode_robot_cost.0
    {
        builds.push(Resource::Ore);
    }

    if state.robots[Resource::Clay as usize] < blueprint.obsidian_robot_cost.1 {
        builds.push(Resource::Clay);
    }

    if state.robots[Resource::Obsidian as usize] < blueprint.geode_robot_cost.1 {
        builds.push(Resource::Obsidian);
    }

    builds
}

fn wait_for_next_robot(blueprint: &Blueprint, state: &mut State, robot_type: Resource) -> bool {
    while state.minutes > 0 {
        state.minutes -= 1;
        let can_build = match robot_type {
            Resource::Ore => blueprint.ore_robot_cost <= state.stash[Resource::Ore as usize],
            Resource::Clay => blueprint.clay_robot_cost <= state.stash[Resource::Ore as usize],
            Resource::Obsidian => {
                blueprint.obsidian_robot_cost.0 <= state.stash[Resource::Ore as usize]
                    && blueprint.obsidian_robot_cost.1 <= state.stash[Resource::Clay as usize]
            }
            Resource::Geode => {
                blueprint.geode_robot_cost.0 <= state.stash[Resource::Ore as usize]
                    && blueprint.geode_robot_cost.1 <= state.stash[Resource::Obsidian as usize]
            }
        };

        if can_build {
            match robot_type {
                Resource::Ore => {
                    state.stash[Resource::Ore as usize] -= blueprint.ore_robot_cost;
                }
                Resource::Clay => {
                    state.stash[Resource::Ore as usize] -= blueprint.clay_robot_cost;
                }
                Resource::Obsidian => {
                    state.stash[Resource::Ore as usize] -= blueprint.obsidian_robot_cost.0;
                    state.stash[Resource::Clay as usize] -= blueprint.obsidian_robot_cost.1;
                }
                Resource::Geode => {
                    state.stash[Resource::Ore as usize] -= blueprint.geode_robot_cost.0;
                    state.stash[Resource::Obsidian as usize] -= blueprint.geode_robot_cost.1;
                }
            }
        }

        // harvest resources
        for i in 0..4 {
            state.stash[i] += state.robots[i];
        }

        if can_build {
            return true;
        }
    }
    false
}

fn parse_input(input: &str) -> impl Iterator<Item = Blueprint> + '_ {
    input
        .lines()
        .enumerate()
        .map(|(i, l)| Blueprint::parse(l, i + 1))
}

impl Day for Day19 {
    fn star1(&self, input: &str) -> String {
        let sum_q: usize = parse_input(input)
            .map(|blueprint| blueprint.id * geodes(&blueprint, 24))
            .sum();
        format!("{}", sum_q)
    }

    fn star2(&self, input: &str) -> String {
        let p: usize = parse_input(input)
            .take(3)
            .map(|blueprint| geodes(&blueprint, 32))
            .product();
        format!("{}", p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."#;

        let d = Day19 {};
        assert_eq!(d.star1(input), "33");
        assert_eq!(d.star2(input), format!("{}", 56 * 62));
    }
}
