use common::day::Day;
use itertools::Itertools;

pub struct Day21 {}

struct Stats {
    hp: i64,
    dmg: i64,
    arm: i64,
}

#[derive(Debug)]
struct Item {
    cost: i64,
    dmg: i64,
    arm: i64,
}

const WEAPONS: [Item; 5] = [
    Item {
        cost: 8,
        dmg: 4,
        arm: 0,
    },
    Item {
        cost: 10,
        dmg: 5,
        arm: 0,
    },
    Item {
        cost: 25,
        dmg: 6,
        arm: 0,
    },
    Item {
        cost: 40,
        dmg: 7,
        arm: 0,
    },
    Item {
        cost: 74,
        dmg: 8,
        arm: 0,
    },
];

const ARMOR: [Item; 6] = [
    Item {
        cost: 0,
        dmg: 0,
        arm: 0,
    },
    Item {
        cost: 13,
        dmg: 0,
        arm: 1,
    },
    Item {
        cost: 31,
        dmg: 0,
        arm: 2,
    },
    Item {
        cost: 53,
        dmg: 0,
        arm: 3,
    },
    Item {
        cost: 75,
        dmg: 0,
        arm: 4,
    },
    Item {
        cost: 102,
        dmg: 0,
        arm: 5,
    },
];

const RINGS: [Item; 8] = [
    Item {
        cost: 0,
        dmg: 0,
        arm: 0,
    },
    Item {
        cost: 0,
        dmg: 0,
        arm: 0,
    },
    Item {
        cost: 25,
        dmg: 1,
        arm: 0,
    },
    Item {
        cost: 50,
        dmg: 2,
        arm: 0,
    },
    Item {
        cost: 100,
        dmg: 3,
        arm: 0,
    },
    Item {
        cost: 20,
        dmg: 0,
        arm: 1,
    },
    Item {
        cost: 40,
        dmg: 0,
        arm: 2,
    },
    Item {
        cost: 80,
        dmg: 0,
        arm: 3,
    },
];

fn combat(player_stats: &Stats, boss_stats: &Stats) -> bool {
    let mut player_cur_hp = player_stats.hp;
    let mut boss_cur_hp = boss_stats.hp;
    let mut turn = 0;
    while boss_cur_hp > 0 && player_cur_hp > 0 {
        if turn % 2 == 0 {
            boss_cur_hp -= (player_stats.dmg - boss_stats.arm).max(0);
        } else {
            player_cur_hp -= (boss_stats.dmg - player_stats.arm).max(0);
        }
        turn += 1;
    }
    player_cur_hp > 0
}

fn min_cost_to_win(boss_stats: &Stats) -> i64 {
    let mut min_cost = std::i64::MAX;
    for w in WEAPONS.iter() {
        for a in ARMOR.iter() {
            for rs in RINGS.iter().combinations(2) {
                let player_stats = Stats {
                    hp: 100,
                    dmg: w.dmg + rs[0].dmg + rs[1].dmg,
                    arm: a.arm + rs[0].arm + rs[1].arm,
                };
                let cost = w.cost + a.cost + rs[0].cost + rs[1].cost;
                if combat(&player_stats, &boss_stats) {
                    min_cost = min_cost.min(cost);
                }
            }
        }
    }
    min_cost
}

fn max_cost_to_lose(boss_stats: &Stats) -> i64 {
    let mut max_cost = std::i64::MIN;
    for w in WEAPONS.iter() {
        for a in ARMOR.iter() {
            for rs in RINGS.iter().combinations(2) {
                let player_stats = Stats {
                    hp: 100,
                    dmg: w.dmg + rs[0].dmg + rs[1].dmg,
                    arm: a.arm + rs[0].arm + rs[1].arm,
                };
                let cost = w.cost + a.cost + rs[0].cost + rs[1].cost;
                if !combat(&player_stats, &boss_stats) {
                    max_cost = max_cost.max(cost);
                }
            }
        }
    }
    max_cost
}

fn parse_input(input: &str) -> Stats {
    let mut lines = input.lines();
    let hp = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .parse::<i64>()
        .unwrap();
    let dmg = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .parse::<i64>()
        .unwrap();
    let arm = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .parse::<i64>()
        .unwrap();
    Stats { hp, dmg, arm }
}

impl Day for Day21 {
    fn star1(&self, input: &str) -> String {
        let boss_stats = parse_input(input);
        format!("{}", min_cost_to_win(&boss_stats))
    }

    fn star2(&self, input: &str) -> String {
        let boss_stats = parse_input(input);
        format!("{}", max_cost_to_lose(&boss_stats))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let player_stats = Stats {
            hp: 8,
            dmg: 5,
            arm: 5,
        };
        let boss_stats = Stats {
            hp: 12,
            dmg: 7,
            arm: 2,
        };
        assert_eq!(combat(&player_stats, &boss_stats), true);
    }
}
