use common::day::Day;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct Day21 {}

struct InputData<'a> {
    ingredients: HashMap<&'a str, HashSet<usize>>,
    allergens: HashMap<&'a str, HashSet<usize>>,
}

fn parse_input(input: &str) -> InputData {
    let mut ingredients = HashMap::new();
    let mut allergens = HashMap::new();
    for (line_no, line) in input.lines().enumerate() {
        let mut spl = line.split(" (contains ");
        let ingred_list = spl.next().unwrap();
        let allergen_list = spl.next().unwrap();
        // skip final closing paren
        let allergen_list = &allergen_list[0..allergen_list.len() - 1];

        let line_ingredients: HashSet<_> = ingred_list.split_whitespace().collect();
        let line_allergens: HashSet<_> = allergen_list.split(", ").collect();

        for ingredient in line_ingredients.iter() {
            let e = ingredients.entry(*ingredient).or_insert_with(HashSet::new);
            e.insert(line_no);
        }

        for allergen in line_allergens.iter() {
            let e = allergens.entry(*allergen).or_insert_with(HashSet::new);
            e.insert(line_no);
        }
    }
    InputData {
        ingredients,
        allergens,
    }
}

fn get_non_allergens<'a>(input_data: &'a InputData) -> HashSet<&'a str> {
    input_data
        .ingredients
        .iter()
        .filter(|(_, is_in)| {
            let possible_allergens = input_data
                .allergens
                .iter()
                .filter(|(_, must_be_in)| must_be_in.iter().all(|ruleno| is_in.contains(ruleno)))
                .count();
            possible_allergens == 0
        })
        .map(|(ingred, _)| *ingred)
        .collect()
}

fn check_match(input_data: &InputData, allergens: &[&str], perm: &[&&&str]) -> bool {
    allergens
        .iter()
        .zip(perm.iter())
        .all(|(allergen, ingredient)| {
            let must_be_in = input_data.allergens.get(allergen).unwrap();
            let is_in = input_data.ingredients.get(**ingredient).unwrap();
            must_be_in.iter().all(|ruleno| is_in.contains(ruleno))
        })
}

impl Day for Day21 {
    fn star1(&self, input: &str) -> String {
        let input_data = parse_input(input);
        let non_allergens = get_non_allergens(&input_data);
        let occur: usize = non_allergens
            .iter()
            .map(|non_allergen| input_data.ingredients.get(non_allergen).unwrap().len())
            .sum();
        format!("{}", occur)
    }

    fn star2(&self, input: &str) -> String {
        let input_data = parse_input(input);
        let allergens: Vec<_> = input_data.allergens.keys().sorted().cloned().collect();

        let non_allergen_ingreds = get_non_allergens(&input_data);
        let all_ingreds: HashSet<_> = input_data.ingredients.keys().cloned().collect();
        let allergen_ingreds: HashSet<_> = all_ingreds.difference(&non_allergen_ingreds).collect();
        for perm in allergen_ingreds.iter().permutations(allergen_ingreds.len()) {
            if check_match(&input_data, &allergens, &perm) {
                return perm.iter().join(",");
            }
        }

        String::from("err")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"#;
        let d = Day21 {};
        assert_eq!(d.star1(input), "5");
        assert_eq!(d.star2(input), "mxmxvkd,sqjhc,fvjkl");
    }
}
