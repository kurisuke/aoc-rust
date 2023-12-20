use std::collections::{HashMap, VecDeque};

use common::day::Day;

pub struct Day20 {}

impl Day for Day20 {
    fn star1(&self, input: &str) -> String {
        let mut modules = parse_input(input);
        let mut low_pulses = 0;
        let mut high_pulses = 0;

        for _ in 0..1000 {
            let ret = process(modules, None).unwrap();
            low_pulses += ret.0;
            high_pulses += ret.1;
            modules = ret.2;
        }
        (low_pulses * high_pulses).to_string()
    }

    fn star2(&self, input: &str) -> String {
        let modules = parse_input(input);

        let rx_feeder = modules
            .values()
            .find(|m| m.outputs.contains(&String::from("rx")))
            .map(|m| m.id.clone())
            .unwrap();

        let rx_feeders_2: Vec<_> = modules
            .values()
            .filter(|m| m.outputs.contains(&rx_feeder))
            .map(|m| m.id.clone())
            .collect();

        // assumptions:
        // - co-prime cycle lengths (GCD = 1)
        // - for each n: x % n = (n - 1)
        // - then the solution is the product of the cycle lengths
        rx_feeders_2
            .into_iter()
            .map(|rx_feeder_2| run_until_true(modules.clone(), &rx_feeder_2))
            .product::<usize>()
            .to_string()
    }
}

type Modules = HashMap<String, Module>;

#[derive(Debug, Clone)]
struct Module {
    id: String,
    type_: ModuleType,
    outputs: Vec<String>,
}

impl Module {
    fn process(&mut self, pulse: Pulse) -> Vec<(String, Pulse)> {
        if let Some(output_val) = self.type_.process(pulse) {
            self.outputs
                .iter()
                .map(|output| {
                    (
                        output.clone(),
                        Pulse {
                            val: output_val,
                            src: self.id.to_string(),
                        },
                    )
                })
                .collect()
        } else {
            vec![]
        }
    }
}

#[derive(Debug, Clone)]
enum ModuleType {
    FlipFlop { state: bool },
    Conjunction { inputs: HashMap<String, bool> },
    Source,
    Sink,
}

impl ModuleType {
    fn process(&mut self, pulse: Pulse) -> Option<bool> {
        match self {
            ModuleType::FlipFlop { state } => {
                if pulse.val {
                    // high
                    None
                } else {
                    *state = !(*state);
                    Some(*state)
                }
            }
            ModuleType::Conjunction { inputs } => {
                inputs.entry(pulse.src).and_modify(|e| *e = pulse.val);
                if inputs.values().all(|x| *x) {
                    Some(false)
                } else {
                    Some(true)
                }
            }
            ModuleType::Source => Some(pulse.val),
            ModuleType::Sink => None,
        }
    }
}

struct Pulse {
    val: bool,
    src: String,
}

fn parse_input(input: &str) -> Modules {
    let mut modules = HashMap::new();

    for line in input.lines() {
        let (id, outputs) = line.split_once(" -> ").unwrap();
        let outputs: Vec<_> = outputs.split(", ").map(|s| s.to_string()).collect();
        let type_ = match id.chars().next().unwrap() {
            '&' => ModuleType::Conjunction {
                inputs: HashMap::new(),
            },
            '%' => ModuleType::FlipFlop { state: false },
            'b' => ModuleType::Source,
            _ => unreachable!(),
        };

        let id = if id == "broadcaster" {
            String::from("broadcaster")
        } else {
            id[1..].to_string()
        };
        modules.insert(id.clone(), Module { id, type_, outputs });
    }

    // check for unconnected outputs
    let unconnected_outputs: Vec<_> = modules
        .values()
        .flat_map(|m| m.outputs.iter().filter(|o| !modules.contains_key(*o)))
        .map(|s| s.to_string())
        .collect();
    for u in unconnected_outputs {
        modules.insert(
            u.clone(),
            Module {
                id: u,
                type_: ModuleType::Sink,
                outputs: vec![],
            },
        );
    }

    // connections to conjunction modules
    let conjunction_connections: Vec<_> = modules
        .values()
        .flat_map(|m| {
            m.outputs
                .iter()
                .filter(|o| {
                    matches!(
                        modules.get(*o).unwrap().type_,
                        ModuleType::Conjunction { inputs: _ }
                    )
                })
                .map(|o| (m.id.to_string(), o.to_string()))
        })
        .collect();

    for (src, dest) in conjunction_connections {
        let m = modules.get_mut(&dest).unwrap();
        match &mut m.type_ {
            ModuleType::Conjunction { inputs } => {
                inputs.insert(src, false);
            }
            _ => unreachable!(),
        }
    }

    modules
}

fn process(
    mut modules: Modules,
    abort_if_high_pulse: Option<&str>,
) -> Option<(usize, usize, Modules)> {
    let mut pulse_queue = VecDeque::new();
    pulse_queue.push_back((
        String::from("broadcaster"),
        Pulse {
            val: false,
            src: String::from("button"),
        },
    ));

    let mut low_pulses = 0;
    let mut high_pulses = 0;

    while let Some((dest, pulse)) = pulse_queue.pop_front() {
        if let Some(abort_if_high_pulse) = abort_if_high_pulse {
            if pulse.src == abort_if_high_pulse && pulse.val {
                return None;
            }
        }
        if pulse.val {
            high_pulses += 1;
        } else {
            low_pulses += 1;
        }

        let module = modules.get_mut(&dest).unwrap();
        for new_pulse in module.process(pulse).into_iter() {
            pulse_queue.push_back(new_pulse);
        }
    }

    Some((low_pulses, high_pulses, modules))
}

fn run_until_true(mut modules: Modules, id: &str) -> usize {
    let mut i = 0;
    loop {
        i += 1;
        match process(modules, Some(id)) {
            Some((_, _, m)) => {
                modules = m;
            }
            None => {
                return i;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#;

        let d = Day20 {};
        assert_eq!(d.star1(input), "32000000");
    }

    #[test]
    fn ex2() {
        let input = r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"#;

        let d = Day20 {};
        assert_eq!(d.star1(input), "11687500");
    }
}
