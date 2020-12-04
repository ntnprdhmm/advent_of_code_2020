use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_entries() -> Vec<HashMap<String, String>> {
  let file = File::open("./inputs/day_4.txt").unwrap();

  let br = BufReader::new(file);

  let mut raw_passports: Vec<String> = Vec::new();
  let mut current_raw_passport: Vec<String> = vec![];

  for line in br.lines() {
    let line = line.unwrap();

    if line == "" {
      raw_passports.push(current_raw_passport.join(" "));
      current_raw_passport = vec![];
    } else {
      current_raw_passport.push(line);
    }
  }
  raw_passports.push(current_raw_passport.join(" "));

  let mut passports: Vec<HashMap<String, String>> = Vec::new();

  for raw_passport in raw_passports {
    let mut passport: HashMap<String, String> = HashMap::new();

    let keys_values = raw_passport.split(" ");
    for key_value in keys_values {
      let parts: Vec<&str> = key_value.split(":").collect();
      passport.insert(String::from(parts[0]), String::from(parts[1]));
    }

    passports.push(passport);
  }

  passports
}

fn is_passport_valid(passport: &HashMap<String, String>) -> bool {
  return passport.contains_key("byr")
    && passport.contains_key("iyr")
    && passport.contains_key("eyr")
    && passport.contains_key("hgt")
    && passport.contains_key("ecl")
    && passport.contains_key("pid")
    && passport.contains_key("hcl");
}

pub fn passport_processing() -> usize {
  let passports = read_entries();

  let mut counter = 0;

  for passport in passports {
    if is_passport_valid(&passport) {
      counter += 1;
    }
  }

  return counter;
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_input() {
    assert_eq!(super::passport_processing(), 206);
  }

  #[test]
  fn test_is_passport_valid_true() {
    let mut hashmap: super::HashMap<String, String> = super::HashMap::new();
    hashmap.insert(String::from("ecl"), String::from("gry"));
    hashmap.insert(String::from("pid"), String::from("860033327"));
    hashmap.insert(String::from("eyr"), String::from("2020"));
    hashmap.insert(String::from("hcl"), String::from("#fffffd"));
    hashmap.insert(String::from("byr"), String::from("#1937"));
    hashmap.insert(String::from("iyr"), String::from("2017"));
    hashmap.insert(String::from("cid"), String::from("147"));
    hashmap.insert(String::from("hgt"), String::from("183cm"));

    assert_eq!(super::is_passport_valid(&hashmap), true);
  }

  #[test]
  fn test_is_passport_valid_false() {
    let mut hashmap: super::HashMap<String, String> = super::HashMap::new();
    hashmap.insert(String::from("iyr"), String::from("2013"));
    hashmap.insert(String::from("ecl"), String::from("amb"));
    hashmap.insert(String::from("cid"), String::from("350"));
    hashmap.insert(String::from("eyr"), String::from("2023"));
    hashmap.insert(String::from("pid"), String::from("028048884"));
    hashmap.insert(String::from("hcl"), String::from("#cfa07d"));
    hashmap.insert(String::from("byr"), String::from("1929"));

    assert_eq!(super::is_passport_valid(&hashmap), false);
  }
}
