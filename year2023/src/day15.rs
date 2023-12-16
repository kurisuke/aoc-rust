use common::day::Day;

pub struct Day15 {}

impl Day for Day15 {
    fn star1(&self, input: &str) -> String {
        let input = input.replace('\n', "");
        input.split(',').map(hash).sum::<usize>().to_string()
    }

    fn star2(&self, input: &str) -> String {
        let mut boxes: [_; 256] = std::array::from_fn(|_| vec![]);

        let input = input.replace('\n', "");
        for s in input.split(',') {
            boxes = insert(boxes, s);
        }

        focusing_power(&boxes).to_string()
    }
}

type Boxes<'a> = [Vec<(&'a str, usize)>; 256];

fn insert<'a>(mut boxes: Boxes<'a>, s: &'a str) -> Boxes<'a> {
    if s.ends_with('-') {
        let label = &s[0..s.len() - 1];
        let target_box = &mut boxes[hash(label)];
        if let Some(p) = target_box.iter().position(|(l, _)| l == &label) {
            target_box.remove(p);
        }
    } else {
        let (label, focal_length) = s.split_once('=').unwrap();
        let focal_length = focal_length.parse().unwrap();
        let target_box = &mut boxes[hash(label)];
        if let Some(p) = target_box.iter().position(|(l, _)| l == &label) {
            target_box[p].1 = focal_length;
        } else {
            target_box.push((label, focal_length));
        }
    }
    boxes
}

fn focusing_power(boxes: &Boxes<'_>) -> usize {
    boxes
        .iter()
        .enumerate()
        .map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(|(j, (_, focal_length))| (i + 1) * (j + 1) * focal_length)
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn hash(s: &str) -> usize {
    let mut h = 0;
    for c in s.chars() {
        h += (c as u8) as usize;
        h *= 17;
        h %= 256;
    }
    h
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn ex1() {
        let d = Day15 {};
        assert_eq!(d.star1(INPUT), "1320");
    }

    #[test]
    fn ex2() {
        let d = Day15 {};
        assert_eq!(d.star2(INPUT), "145");
    }

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }
}
