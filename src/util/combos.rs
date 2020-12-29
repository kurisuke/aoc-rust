use std::cmp::Ordering;

pub fn get_combos(target: usize, containers: &[usize]) -> Vec<Vec<usize>> {
    let mut combos = vec![];
    for (idx, container) in containers.iter().enumerate() {
        match target.cmp(container) {
            Ordering::Equal => {
                combos.push(vec![*container]);
            }
            Ordering::Greater => {
                let new_target = target - container;
                let new_containers: Vec<usize> = containers[idx + 1..].iter().copied().collect();
                for sub_combo in get_combos(new_target, &new_containers) {
                    let mut combo = vec![*container];
                    combo.extend(&sub_combo);
                    combos.push(combo);
                }
            }
            Ordering::Less => {}
        }
    }
    combos
}
