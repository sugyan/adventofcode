use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

struct Solution {
    inputs: Vec<String>,
}

impl Solution {
    fn new(inputs: Vec<String>) -> Self {
        Self { inputs }
    }
    fn part_1(&self) -> usize {
        self.count_valid(false)
    }
    fn part_2(&self) -> usize {
        self.count_valid(true)
    }
    fn count_valid(&self, validate_value: bool) -> usize {
        self.inputs
            .split(String::is_empty)
            .filter(|&lines| {
                let fields = lines
                    .iter()
                    .map(|line| {
                        line.split(' ').map(|field| {
                            let v = field.split(':').collect::<Vec<_>>();
                            (v[0].to_string(), v[1].to_string())
                        })
                    })
                    .flatten()
                    .collect::<HashMap<_, _>>();
                (fields.len() == 8 || (fields.len() == 7 && !fields.contains_key("cid")))
                    && (!validate_value || self.validate_values(&fields))
            })
            .count()
    }
    fn validate_values(&self, fields: &HashMap<String, String>) -> bool {
        fields.iter().all(|(key, value)| match key.as_str() {
            "byr" => value
                .parse::<i32>()
                .map_or(false, |y| (1920..=2002).contains(&y)),
            "iyr" => value
                .parse::<i32>()
                .map_or(false, |y| (2010..=2020).contains(&y)),
            "eyr" => value
                .parse::<i32>()
                .map_or(false, |y| (2020..=2030).contains(&y)),

            "hgt" => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
                }
                if let Some(cap) = RE.captures_iter(value).next() {
                    let n: i32 = cap[1].parse::<i32>().unwrap();
                    match &cap[2] {
                        "cm" => (150..=193).contains(&n),
                        "in" => (59..=76).contains(&n),
                        _ => false,
                    }
                } else {
                    false
                }
            }
            "hcl" => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
                }
                RE.is_match(value)
            }
            "ecl" => matches!(
                value.as_str(),
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
            ),
            "pid" => value.len() == 9 && value.chars().all(char::is_numeric),
            "cid" => true,
            _ => false,
        })
    }
}

fn main() {
    let solution = Solution::new(
        BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(|line| line.ok())
            .collect(),
    );
    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            2,
            Solution::new(
                r"
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"
                    .split('\n')
                    .skip(1)
                    .map(str::to_string)
                    .collect()
            )
            .part_1()
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            0,
            Solution::new(
                r"
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"
                    .split('\n')
                    .skip(1)
                    .map(str::to_string)
                    .collect()
            )
            .part_2()
        );
        assert_eq!(
            4,
            Solution::new(
                r"
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
                    .split('\n')
                    .skip(1)
                    .map(str::to_string)
                    .collect()
            )
            .part_2()
        );
    }
}
