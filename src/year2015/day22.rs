use crate::day::Day;

pub struct Day22 {}

#[derive(PartialEq, Clone, Debug)]
enum Action {
    Missile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

#[derive(PartialEq)]
enum CombatState {
    Ongoing,
    Won,
    Lost,
}

#[derive(Clone)]
struct Stats {
    hp: i64,
    dmg: i64,
    arm: i64,
}

#[derive(Clone)]
struct GameState {
    mana: i64,
    player_stats: Stats,
    boss_stats: Stats,
}

fn mana_cost(action: &Action) -> i64 {
    match action {
        Action::Missile => 53,
        Action::Drain => 73,
        Action::Shield => 113,
        Action::Poison => 173,
        Action::Recharge => 229,
    }
}

fn do_missile(mut state: &mut GameState) {
    state.boss_stats.hp -= 4;
}

fn do_drain(mut state: &mut GameState) {
    state.boss_stats.hp -= 2;
    state.player_stats.hp += 2;
}

fn start_shield(mut state: &mut GameState) {
    state.player_stats.arm = 7;
}

fn stop_shield(mut state: &mut GameState) {
    state.player_stats.arm = 0;
}

fn do_poison(mut state: &mut GameState) {
    state.boss_stats.hp -= 3;
}

fn do_recharge(mut state: &mut GameState) {
    state.mana += 101;
}

fn do_boss_attack(mut state: &mut GameState) {
    state.player_stats.hp -= (state.boss_stats.dmg - state.player_stats.arm).max(0);
}

fn perform_action(
    action: &Action,
    active_effects: &mut Vec<(Action, usize)>,
    state: &mut GameState,
) {
    state.mana -= mana_cost(action);
    match action {
        Action::Missile => {
            do_missile(state);
        }
        Action::Drain => {
            do_drain(state);
        }
        Action::Shield => {
            start_shield(state);
            active_effects.push((Action::Shield, 6));
        }
        Action::Poison => {
            active_effects.push((Action::Poison, 6));
        }
        Action::Recharge => {
            active_effects.push((Action::Recharge, 5));
        }
    }
}

fn do_effects(active_effects: &mut Vec<(Action, usize)>, state: &mut GameState) {
    for effect in active_effects.iter_mut() {
        effect.1 -= 1;
        match effect.0 {
            Action::Shield => {
                if effect.1 == 0 {
                    stop_shield(state);
                }
            }
            Action::Poison => {
                do_poison(state);
            }
            Action::Recharge => {
                do_recharge(state);
            }
            _ => {}
        }
    }
    active_effects.retain(|x| x.1 > 0);
}

fn combat_finished(state: &GameState) -> CombatState {
    if state.player_stats.hp > 0 && state.boss_stats.hp > 0 {
        CombatState::Ongoing
    } else if state.player_stats.hp <= 0 {
        CombatState::Lost
    } else {
        CombatState::Won
    }
}

fn possible_actions(active_effects: &[(Action, usize)], state: &GameState) -> Vec<Action> {
    let mut actions = vec![];

    // Missile
    if state.mana >= mana_cost(&Action::Missile) {
        actions.push(Action::Missile);
    }

    // Drain
    if state.mana >= mana_cost(&Action::Drain) {
        actions.push(Action::Drain);
    }

    // Shield
    if !active_effects.iter().any(|a| a.0 == Action::Shield)
        && state.mana >= mana_cost(&Action::Shield)
    {
        actions.push(Action::Shield);
    }

    // Poison
    if !active_effects.iter().any(|a| a.0 == Action::Poison)
        && state.mana >= mana_cost(&Action::Poison)
    {
        actions.push(Action::Poison);
    }

    // Recharge
    if !active_effects.iter().any(|a| a.0 == Action::Recharge)
        && state.mana >= mana_cost(&Action::Recharge)
    {
        actions.push(Action::Recharge);
    }

    actions
}

fn early_exit(prev_actions: &[Action], state: &GameState, win_seqs: &mut Vec<Vec<Action>>) -> bool {
    if combat_finished(&state) != CombatState::Ongoing {
        if combat_finished(&state) == CombatState::Won {
            win_seqs.push(prev_actions.to_vec());
        }
        true
    } else {
        false
    }
}

fn next_action(
    prev_actions: Vec<Action>,
    mut active_effects: Vec<(Action, usize)>,
    mut state: GameState,
    win_seqs: &mut Vec<Vec<Action>>,
    lose_at_turn_start: i64,
) {
    if prev_actions.len() > 10 {
        return;
    }

    perform_action(
        prev_actions.last().unwrap(),
        &mut active_effects,
        &mut state,
    );
    if early_exit(&prev_actions, &state, win_seqs) {
        return;
    }

    // begin of boss turn
    state.player_stats.hp -= lose_at_turn_start;
    if early_exit(&prev_actions, &state, win_seqs) {
        return;
    }

    do_effects(&mut active_effects, &mut state);
    if early_exit(&prev_actions, &state, win_seqs) {
        return;
    }

    do_boss_attack(&mut state);
    if early_exit(&prev_actions, &state, win_seqs) {
        return;
    }

    // begin of player turn
    state.player_stats.hp -= lose_at_turn_start;
    if early_exit(&prev_actions, &state, win_seqs) {
        return;
    }

    do_effects(&mut active_effects, &mut state);
    if early_exit(&prev_actions, &state, win_seqs) {
        return;
    }

    // decide on next action and run recursively
    for action in possible_actions(&active_effects, &state) {
        let mut new_prev_actions = prev_actions.clone().to_vec();
        new_prev_actions.push(action);
        let new_active_effects = active_effects.clone();
        let new_state = state.clone();
        next_action(
            new_prev_actions,
            new_active_effects,
            new_state,
            win_seqs,
            lose_at_turn_start,
        );
    }
}

fn calc_mana(actions: &[Action]) -> i64 {
    actions.iter().map(mana_cost).sum()
}

fn run(boss_stats: Stats, lose_at_turn_start: i64) -> i64 {
    let init_actions = vec![
        Action::Missile,
        Action::Drain,
        Action::Shield,
        Action::Poison,
        Action::Recharge,
    ];
    let init_state = GameState {
        mana: 500,
        player_stats: Stats {
            hp: 50 - lose_at_turn_start,
            dmg: 0,
            arm: 0,
        },
        boss_stats,
    };
    let mut win_seqs = vec![];
    for init_action in init_actions {
        next_action(
            vec![init_action],
            vec![],
            init_state.clone(),
            &mut win_seqs,
            lose_at_turn_start,
        );
    }
    win_seqs.iter().map(|x| calc_mana(x)).min().unwrap()
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
    Stats { hp, dmg, arm: 0 }
}

impl Day for Day22 {
    fn star1(&self, input: &str) -> String {
        let boss_stats = parse_input(input);
        format!("{}", run(boss_stats, 0))
    }

    fn star2(&self, input: &str) -> String {
        let boss_stats = parse_input(input);
        format!("{}", run(boss_stats, 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let init_actions = vec![
            Action::Missile,
            Action::Drain,
            Action::Shield,
            Action::Poison,
            Action::Recharge,
        ];
        let init_state = GameState {
            mana: 250,
            player_stats: Stats {
                hp: 10,
                dmg: 0,
                arm: 0,
            },
            boss_stats: Stats {
                hp: 14,
                dmg: 8,
                arm: 0,
            },
        };
        let mut win_seqs = vec![];
        for init_action in init_actions {
            next_action(
                vec![init_action],
                vec![],
                init_state.clone(),
                &mut win_seqs,
                0,
            );
        }
        let min_mana = win_seqs.iter().map(|x| calc_mana(x)).min().unwrap();
        assert_eq!(min_mana, 641);
    }
}
