use common::day::Day;
use std::collections::{HashSet, VecDeque};
use util::intcode::{Intcode,IntSize};

pub struct Day23 {}

#[derive(Copy,Clone)]
struct Packet {
    dest: IntSize,
    x: IntSize,
    y: IntSize,
}

fn sim_network(input: &str, part1: bool) -> IntSize {
    let mut comps = vec![Intcode::new_from_str(input); 50];

    // bootup
    for (i, comp) in comps.iter_mut().enumerate() {
        comp.write_inp(i as IntSize);
        comp.run();
    }

    let mut nat_entry = None;
    let mut last_nat_y = None;

    loop {

        // send phase
        let mut send_queue = VecDeque::new();
        for comp in comps.iter_mut() {
            while let Some(dest) = comp.read_outp() {
                let x = comp.read_outp().unwrap();
                let y = comp.read_outp().unwrap();
                if dest == 255 {
                    if part1 {
                        return y;
                    }
                    nat_entry = Some(Packet { dest, x, y });
                } else {
                    send_queue.push_back(Packet { dest, x, y });
                }
            }
        }

        // recv phase; fill packet queues
        let mut written_to = HashSet::new();
        while let Some(p) = send_queue.pop_front() {
            comps[p.dest as usize].write_inp(p.x);
            comps[p.dest as usize].write_inp(p.y);
            written_to.insert(p.dest as usize);
        }

        if written_to.is_empty() && nat_entry.is_some() {
            // network is idle
            let ne = nat_entry.unwrap();
            comps[0].write_inp(ne.x);
            comps[0].write_inp(ne.y);
            if last_nat_y == Some(ne.y) {
                return ne.y;
            }
            last_nat_y = Some(ne.y);
            nat_entry = None;

            // write -1 to all other computers
            for comp in comps.iter_mut().skip(1) {
                comp.write_inp(-1);
            }
        } else {
            // write -1 (no input) to all computers not written to
            for (i, comp) in comps.iter_mut().enumerate() {
                if !written_to.contains(&i) {
                    comp.write_inp(-1);
                }
            }
        }

        for comp in comps.iter_mut() {
            comp.run();
        }
    }
}

impl Day for Day23 {
    fn star1(&self, input: &str) -> String {
        format!("{}", sim_network(input, true))
    }

    fn star2(&self, input: &str) -> String {
        format!("{}", sim_network(input, false))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day23 {};
        assert_eq!(d.star1(""), "not implemented");
    }
}
