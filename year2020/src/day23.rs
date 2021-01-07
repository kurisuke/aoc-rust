use common::day::Day;

pub struct Day23 {}

fn make_linked(cups: Vec<u32>) -> Vec<u32> {
    // make a "linked list"
    // the value at idx(cup no.) is the cup number of the next cup in the circle
    let mut cups_linked = vec![0; cups.len() + 1];
    for window in cups.windows(3) {
        cups_linked[window[1] as usize] = window[2];
    }
    // first cup is linked to the second
    cups_linked[cups[0] as usize] = cups[1];
    // last cup is linked to the first
    cups_linked[cups[cups.len() - 1] as usize] = cups[0];

    // note: cups_linked[0] stays empty and is never referenced
    cups_linked
}

fn mv_cups_ll(cups_linked: &mut Vec<u32>, cur_cup: &mut u32) {
    // find the 3 cups following the current cup (they will be moved)
    let mv1 = cups_linked[*cur_cup as usize];
    let mv2 = cups_linked[mv1 as usize];
    let mv3 = cups_linked[mv2 as usize];

    // find the destination cup after which to insert the moved cups
    let mut dest_cup = *cur_cup;
    while dest_cup == *cur_cup || dest_cup == mv1 || dest_cup == mv2 || dest_cup == mv3 {
        dest_cup = if dest_cup == 1 {
            cups_linked.len() as u32 - 1
        } else {
            dest_cup - 1
        };
    }

    // "unlink" the 3 cups after current
    cups_linked[*cur_cup as usize] = cups_linked[mv3 as usize];

    // insert the grabbed cups after the destination cup
    let after_insert = cups_linked[dest_cup as usize];
    cups_linked[dest_cup as usize] = mv1;
    cups_linked[mv3 as usize] = after_insert;

    // advance the current cup for the next round
    *cur_cup = cups_linked[*cur_cup as usize];
}

fn order_str(cups_linked: &[u32]) -> String {
    let mut s = String::new();
    let mut i = 1;
    loop {
        // go through the order until we come back to cup no. 1
        i = cups_linked[i as usize];
        if i == 1 {
            break;
        }
        // add cup number to string
        s.push_str(&format!("{}", i));
    }
    s
}

impl Day for Day23 {
    fn star1(&self, input: &str) -> String {
        let cups: Vec<_> = input.chars().map(|x| x.to_digit(10).unwrap()).collect();
        let mut cur_cup = cups[0];
        let mut cups_linked = make_linked(cups);
        for _ in 0..100 {
            mv_cups_ll(&mut cups_linked, &mut cur_cup);
        }
        order_str(&cups_linked)
    }

    fn star2(&self, input: &str) -> String {
        let mut cups: Vec<_> = input.chars().map(|x| x.to_digit(10).unwrap()).collect();
        for i in 10..=1_000_000 {
            cups.push(i);
        }
        let mut cur_cup = cups[0];
        let mut cups_linked = make_linked(cups);
        for _ in 0..10_000_000 {
            mv_cups_ll(&mut cups_linked, &mut cur_cup);
        }
        let next1 = cups_linked[1];
        let next2 = cups_linked[next1 as usize];
        format!("{}", next1 as u64 * next2 as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = "389125467";
        let d = Day23 {};
        assert_eq!(d.star1(input), "67384529");
        assert_eq!(d.star2(input), "149245887792");
    }
}
