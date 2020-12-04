#[macro_use]
extern crate lazy_static;

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
    fn solve_1(&self) -> usize {
        self.validate(false)
    }
    fn solve_2(&self) -> usize {
        self.validate(true)
    }
    fn validate(&self, validate_value: bool) -> usize {
        let mut ret = 0;
        let mut fields: HashMap<String, String> = HashMap::new();
        for line in self.inputs.iter().chain([String::new()].iter()) {
            if line.is_empty() {
                if (fields.len() == 8 || (fields.len() == 7 && !fields.contains_key("cid")))
                    && (!validate_value
                        || fields.iter().all(|(key, value)| match key.as_str() {
                            "byr" => {
                                if let Ok(y) = value.parse::<i32>() {
                                    1920 <= y && y <= 2002
                                } else {
                                    false
                                }
                            }
                            "iyr" => {
                                if let Ok(y) = value.parse::<i32>() {
                                    2010 <= y && y <= 2020
                                } else {
                                    false
                                }
                            }
                            "eyr" => {
                                if let Ok(y) = value.parse::<i32>() {
                                    2020 <= y && y <= 2030
                                } else {
                                    false
                                }
                            }
                            "hgt" => {
                                lazy_static! {
                                    static ref RE: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
                                }
                                if let Some(cap) = RE.captures_iter(value).next() {
                                    let n: i32 = cap[1].parse::<i32>().unwrap();
                                    match &cap[2] {
                                        "cm" => 150 <= n && n <= 193,
                                        "in" => 59 <= n && n <= 76,
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
                            "pid" => value.len() == 9 && value.chars().all(|c| c.is_numeric()),
                            "cid" => true,
                            _ => false,
                        }))
                {
                    ret += 1;
                }
                fields.clear();
            } else {
                fields.extend(line.split(' ').map(|field| {
                    let v: Vec<&str> = field.split(':').collect();
                    (v[0].to_string(), v[1].to_string())
                }));
            }
        }
        ret
    }
}

fn main() {
    let solution = Solution::new(
        BufReader::new(std::io::stdin().lock())
            .lines()
            .filter_map(|line| line.ok())
            .collect(),
    );
    println!("{}", solution.solve_1());
    println!("{}", solution.solve_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            2,
            Solution::new(
                "
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"[1..]
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect()
            )
            .solve_1()
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            0,
            Solution::new(
                "
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"[1..]
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect()
            )
            .solve_2()
        );
        assert_eq!(
            4,
            Solution::new(
                "
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"[1..]
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect()
            )
            .solve_2()
        );
    }
}
