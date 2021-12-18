use common::day::Day;
use util::chardistrib;

pub struct Day08 {}

fn get_layers(input: &str, width: usize, height: usize) -> Vec<&str> {
    let layer_size = width * height;
    let mut r = vec![];
    let mut i = 0;
    while i < input.len() {
        r.push(&input[i..i + layer_size]);
        i += layer_size;
    }
    r
}

fn get_min_zero_layer(input: &str, width: usize, height: usize) -> usize {
    let layers = get_layers(input, width, height);
    let min_distrib = layers
        .iter()
        .map(|l| chardistrib::char_distribution(l))
        .min_by(|d1, d2| d1.get(&'0').unwrap_or(&0).cmp(d2.get(&'0').unwrap_or(&0)))
        .unwrap();
    min_distrib.get(&'1').unwrap_or(&0) * min_distrib.get(&'2').unwrap_or(&0)
}

fn decode(input: &str, width: usize, height: usize) -> String {
    let layer_size = width * height;
    let mut pixel_layers = vec![vec![]; layer_size];

    for (i, c) in input.chars().enumerate() {
        pixel_layers[i % layer_size].push(c);
    }

    let pixels: Vec<char> = pixel_layers
        .iter()
        .map(|pl| pl.iter().find(|&&l| l != '2').unwrap_or(&'2'))
        .map(|c| match c {
            '0' => ' ',
            '1' => '█',
            '2' => '.',
            _ => unreachable!(),
        })
        .collect();

    pixels
        .chunks(width)
        .map(|cs| cs.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

impl Day for Day08 {
    fn star1(&self, input: &str) -> String {
        format!("{}", get_min_zero_layer(input.trim(), 25, 6))
    }

    fn star2(&self, input: &str) -> String {
        decode(input, 25, 6)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(get_min_zero_layer("123456789012", 3, 2), 1);
    }
}
