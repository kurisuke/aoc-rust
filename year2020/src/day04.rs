use common::day::Day;
use std::collections::{HashMap, HashSet};

pub struct Day04 {}

impl Day for Day04 {
    fn star1(&self, input: &str) -> String {
        let passports = parse_input(input);
        format!(
            "{}",
            passports.iter().filter(|p| has_valid_fields(p)).count()
        )
    }

    fn star2(&self, input: &str) -> String {
        let passports = parse_input(input);
        format!(
            "{}",
            passports
                .iter()
                .filter(|p| has_valid_fields(p) && field_format_correct(p))
                .count()
        )
    }
}

fn has_valid_fields(passport: &HashMap<&str, &str>) -> bool {
    let req_keys: HashSet<_> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .into_iter()
        .collect();
    let found_keys: HashSet<_> = passport.keys().copied().collect();

    let both_keys_num = req_keys.intersection(&found_keys).count();
    both_keys_num == req_keys.len()
}

fn field_format_correct(passport: &HashMap<&str, &str>) -> bool {
    let byr_ok = check_yr(passport["byr"], 1920, 2002);
    let iyr_ok = check_yr(passport["iyr"], 2010, 2020);
    let eyr_ok = check_yr(passport["eyr"], 2020, 2030);
    let hgt_ok = check_hgt(passport["hgt"]);
    let hcl_ok = check_hcl(passport["hcl"]);

    let valid_ecl: HashSet<_> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .into_iter()
        .collect();
    let ecl_ok = valid_ecl.contains(&passport["ecl"]);

    let pid_ok = check_pid(passport["pid"]);

    byr_ok && iyr_ok && eyr_ok && hgt_ok && hcl_ok && ecl_ok && pid_ok
}

fn check_yr(yr_str: &str, yr_min: u16, yr_max: u16) -> bool {
    if let Ok(yr) = yr_str.parse::<u16>() {
        yr >= yr_min && yr <= yr_max
    } else {
        false
    }
}

fn check_hgt(hgt_str: &str) -> bool {
    if hgt_str.len() < 3 {
        false
    } else {
        let unit = &hgt_str[hgt_str.len() - 2..];
        if unit == "cm" || unit == "in" {
            let val = &hgt_str[..hgt_str.len() - 2];
            if let Ok(val) = val.parse::<u16>() {
                match unit {
                    "cm" => (150..=193).contains(&val),
                    "in" => (59..=76).contains(&val),
                    _ => false,
                }
            } else {
                false
            }
        } else {
            false
        }
    }
}

fn check_hcl(hcl_str: &str) -> bool {
    if hcl_str.len() != 7 || !hcl_str.starts_with('#') {
        false
    } else {
        hcl_str[1..]
            .chars()
            .all(|c| matches!(c, 'a'..='f' | '0'..='9'))
    }
}

fn check_pid(pid_str: &str) -> bool {
    if pid_str.len() == 9 {
        pid_str.parse::<u64>().is_ok()
    } else {
        false
    }
}

fn parse_input(input: &str) -> Vec<HashMap<&str, &str>> {
    input
        .split("\n\n")
        .map(|e| {
            let fields_map: HashMap<_, _> = e
                .split_whitespace()
                .map(|f| {
                    let mut fi = f.split(':');
                    let key = fi.next().unwrap();
                    let value = fi.next().unwrap();
                    (key, value)
                })
                .collect();
            fields_map
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;
        let d = Day04 {};
        assert_eq!(d.star1(input), "2");
    }

    #[test]
    fn ex2() {
        let input = r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
"#;
        let d = Day04 {};
        assert_eq!(d.star2(input), "0");
    }

    #[test]
    fn ex3() {
        let input = r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#;
        let d = Day04 {};
        assert_eq!(d.star2(input), "4");
    }

    #[test]
    fn test_check_hgt() {
        assert!(!check_hgt("58in"));
        assert!(check_hgt("59in"));
        assert!(check_hgt("193cm"));
        assert!(!check_hgt("194cm"));
        assert!(!check_hgt("gibberish"));
        assert!(!check_hgt(""));
    }

    #[test]
    fn test_check_yr() {
        assert!(!check_yr("1919", 1920, 2002));
        assert!(check_yr("1920", 1920, 2002));
        assert!(check_yr("2002", 1920, 2002));
        assert!(!check_yr("2003", 1920, 2002));
        assert!(!check_yr("", 1920, 2002));
        assert!(!check_yr("gibberish", 1920, 2002));
        assert!(!check_yr("2002c", 1920, 2002));
    }

    #[test]
    fn test_check_hcl() {
        assert!(check_hcl("#abc123"));
        assert!(!check_hcl("#abcxyz"));
        assert!(!check_hcl("#abc"));
        assert!(!check_hcl("invalid"));
    }

    #[test]
    fn test_check_pid() {
        assert!(!check_pid("gibberish"));
        assert!(!check_pid("123"));
        assert!(!check_pid("123123123123"));
        assert!(check_pid("123456789"));
        assert!(check_pid("000000123"));
        assert!(!check_pid("      123"));
    }
}
