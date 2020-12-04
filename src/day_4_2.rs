use regex::Regex;
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

fn is_date_value_between_dates(date_value_option: Option<&String>, from: usize, to: usize) -> bool {
  match date_value_option {
    Some(date_value) => {
      let date: usize = date_value.parse().unwrap();
      return date >= from && date <= to;
    }
    None => return false,
  }
}

fn is_height_valid(height_option: Option<&String>) -> bool {
  match height_option {
    Some(height) => {
      let mut height = height.clone();

      let unit = height.pop().unwrap();
      height.pop();

      let number: usize = height.parse().unwrap();

      if unit == 'm' {
        return number >= 150 && number <= 193;
      }

      return number >= 59 && number <= 76;
    }
    None => return false,
  }
}

fn is_hair_color_valid(hair_color_option: Option<&String>) -> bool {
  let re = Regex::new(r"^#[\da-f]{6}$").unwrap();

  match hair_color_option {
    Some(hair_color) => return re.is_match(hair_color),
    None => return false,
  }
}

fn is_eye_color_valid(eye_color_option: Option<&String>) -> bool {
  let eye_colors: HashSet<&'static str> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
    .iter()
    .cloned()
    .collect();

  match eye_color_option {
    Some(eye_color) => {
      let ec: &str = eye_color;
      return eye_colors.contains(ec);
    }
    None => return false,
  }
}

fn is_passport_id_valid(passport_id_option: Option<&String>) -> bool {
  let re = Regex::new(r"^\d{9}$").unwrap();

  match passport_id_option {
    Some(passport_id) => return re.is_match(passport_id),
    None => return false,
  }
}

fn is_passport_valid(passport: &HashMap<String, String>) -> bool {
  return is_date_value_between_dates(passport.get(&String::from("byr")), 1920, 2002)
    && is_date_value_between_dates(passport.get(&String::from("iyr")), 2010, 2020)
    && is_date_value_between_dates(passport.get(&String::from("eyr")), 2020, 2030)
    && is_height_valid(passport.get(&String::from("hgt")))
    && is_hair_color_valid(passport.get(&String::from("hcl")))
    && is_eye_color_valid(passport.get(&String::from("ecl")))
    && is_passport_id_valid(passport.get(&String::from("pid")));
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
    assert_eq!(super::passport_processing(), 123);
  }

  #[test]
  fn test_is_passport_valid_true() {
    let mut passport_1: super::HashMap<String, String> = super::HashMap::new();
    passport_1.insert(String::from("pid"), String::from("087499704"));
    passport_1.insert(String::from("hgt"), String::from("74in"));
    passport_1.insert(String::from("ecl"), String::from("grn"));
    passport_1.insert(String::from("iyr"), String::from("2012"));
    passport_1.insert(String::from("eyr"), String::from("2030"));
    passport_1.insert(String::from("byr"), String::from("1980"));
    passport_1.insert(String::from("hcl"), String::from("#623a2f"));

    let mut passport_2: super::HashMap<String, String> = super::HashMap::new();
    passport_2.insert(String::from("eyr"), String::from("2029"));
    passport_2.insert(String::from("ecl"), String::from("blu"));
    passport_2.insert(String::from("cid"), String::from("129"));
    passport_2.insert(String::from("byr"), String::from("1989"));
    passport_2.insert(String::from("iyr"), String::from("2014"));
    passport_2.insert(String::from("pid"), String::from("896056539"));
    passport_2.insert(String::from("hcl"), String::from("#a97842"));
    passport_2.insert(String::from("hgt"), String::from("165cm"));

    assert_eq!(super::is_passport_valid(&passport_1), true);
    assert_eq!(super::is_passport_valid(&passport_2), true);
  }

  #[test]
  fn test_is_passport_valid_false() {
    let mut passport_1: super::HashMap<String, String> = super::HashMap::new();
    passport_1.insert(String::from("eyr"), String::from("1972"));
    passport_1.insert(String::from("cid"), String::from("100"));
    passport_1.insert(String::from("hcl"), String::from("#18171d"));
    passport_1.insert(String::from("ecl"), String::from("amb"));
    passport_1.insert(String::from("hgt"), String::from("170"));
    passport_1.insert(String::from("pid"), String::from("186cm"));
    passport_1.insert(String::from("iyr"), String::from("2018"));
    passport_1.insert(String::from("byr"), String::from("1926"));

    let mut passport_2: super::HashMap<String, String> = super::HashMap::new();
    passport_2.insert(String::from("eyr"), String::from("1967"));
    passport_2.insert(String::from("hcl"), String::from("#602927"));
    passport_2.insert(String::from("ecl"), String::from("grn"));
    passport_2.insert(String::from("hgt"), String::from("170cm"));
    passport_2.insert(String::from("pid"), String::from("012533040"));
    passport_2.insert(String::from("iyr"), String::from("2019"));
    passport_2.insert(String::from("byr"), String::from("1946"));

    let mut passport_3: super::HashMap<String, String> = super::HashMap::new();
    passport_3.insert(String::from("hcl"), String::from("dab227"));
    passport_3.insert(String::from("iyr"), String::from("2012"));
    passport_3.insert(String::from("ecl"), String::from("brn"));
    passport_3.insert(String::from("hgt"), String::from("182cm"));
    passport_3.insert(String::from("pid"), String::from("021572410"));
    passport_3.insert(String::from("eyr"), String::from("2020"));
    passport_3.insert(String::from("byr"), String::from("1992"));
    passport_3.insert(String::from("cid"), String::from("277"));

    let mut passport_4: super::HashMap<String, String> = super::HashMap::new();
    passport_4.insert(String::from("hgt"), String::from("59cm"));
    passport_4.insert(String::from("ecl"), String::from("zzz"));
    passport_4.insert(String::from("eyr"), String::from("2038"));
    passport_4.insert(String::from("hcl"), String::from("74454a"));
    passport_4.insert(String::from("iyr"), String::from("2023"));
    passport_4.insert(String::from("pid"), String::from("3556412378"));
    passport_4.insert(String::from("byr"), String::from("2007"));

    assert_eq!(super::is_passport_valid(&passport_1), false);
    assert_eq!(super::is_passport_valid(&passport_2), false);
    assert_eq!(super::is_passport_valid(&passport_3), false);
    assert_eq!(super::is_passport_valid(&passport_4), false);
  }

  #[test]
  fn test_is_passport_id_valid() {
    assert_eq!(super::is_passport_id_valid(None), false);
    assert_eq!(
      super::is_passport_id_valid(Some(&String::from("000000001"))),
      true
    );
    assert_eq!(
      super::is_passport_id_valid(Some(&String::from("0123456789"))),
      false
    );
    assert_eq!(
      super::is_passport_id_valid(Some(&String::from("123456789"))),
      true
    );
  }

  #[test]
  fn test_is_hair_color_valid() {
    assert_eq!(super::is_hair_color_valid(None), false);
    assert_eq!(
      super::is_hair_color_valid(Some(&String::from("#123abc"))),
      true
    );
    assert_eq!(
      super::is_hair_color_valid(Some(&String::from("#123abz"))),
      false
    );
    assert_eq!(
      super::is_hair_color_valid(Some(&String::from("123abc"))),
      false
    );
  }

  #[test]
  fn test_is_height_valid() {
    assert_eq!(super::is_height_valid(None), false);
    assert_eq!(super::is_height_valid(Some(&String::from("149cm"))), false);
    assert_eq!(super::is_height_valid(Some(&String::from("150cm"))), true);
    assert_eq!(super::is_height_valid(Some(&String::from("175cm"))), true);
    assert_eq!(super::is_height_valid(Some(&String::from("193cm"))), true);
    assert_eq!(super::is_height_valid(Some(&String::from("194cm"))), false);
    assert_eq!(super::is_height_valid(Some(&String::from("58in"))), false);
    assert_eq!(super::is_height_valid(Some(&String::from("59in"))), true);
    assert_eq!(super::is_height_valid(Some(&String::from("62in"))), true);
    assert_eq!(super::is_height_valid(Some(&String::from("76in"))), true);
    assert_eq!(super::is_height_valid(Some(&String::from("77in"))), false);
  }

  #[test]
  fn test_is_date_value_between_dates() {
    assert_eq!(super::is_date_value_between_dates(None, 2010, 2020), false);
    assert_eq!(
      super::is_date_value_between_dates(Some(&String::from("2009")), 2010, 2020),
      false
    );
    assert_eq!(
      super::is_date_value_between_dates(Some(&String::from("2010")), 2010, 2020),
      true
    );
    assert_eq!(
      super::is_date_value_between_dates(Some(&String::from("2012")), 2010, 2020),
      true
    );
    assert_eq!(
      super::is_date_value_between_dates(Some(&String::from("2020")), 2010, 2020),
      true
    );
    assert_eq!(
      super::is_date_value_between_dates(Some(&String::from("2021")), 2010, 2020),
      false
    );
  }
}
