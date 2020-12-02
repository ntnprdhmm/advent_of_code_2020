use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct PasswordInput {
  password: String,
  letter: char,
  min_occ: u32,
  max_occ: u32,
}

fn read_entries() -> Vec<PasswordInput> {
  let file = File::open("./inputs/day_2.txt").unwrap();

  let br = BufReader::new(file);
  let mut entries: Vec<PasswordInput> = Vec::new();

  for line in br.lines() {
    let line = line.unwrap();
    let line_parts: Vec<&str> = line.trim().split_whitespace().collect();

    let min_occ_string = line_parts[0].split("-").nth(0).unwrap().to_string();
    let max_occ_string = line_parts[0].split("-").nth(1).unwrap().to_string();

    let password: String = line_parts[2].into();

    let letter: char = line_parts[1]
      .split(":")
      .nth(0)
      .unwrap()
      .chars()
      .nth(0)
      .unwrap();

    let password_input: PasswordInput = PasswordInput {
      min_occ: min_occ_string.parse::<u32>().unwrap(),
      max_occ: max_occ_string.parse::<u32>().unwrap(),
      password: password,
      letter: letter,
    };

    entries.push(password_input);
  }

  entries
}

pub fn password_philosophy() -> i32 {
  let entries = read_entries();

  let mut counter = 0;

  for entry in entries {
    let mut letter_occ = 0;
    for c in entry.password.chars() {
      if c == entry.letter {
        letter_occ += 1;
      }
    }

    if letter_occ <= entry.max_occ && letter_occ >= entry.min_occ {
      counter += 1;
    }
  }

  return counter;
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_input() {
    assert_eq!(super::password_philosophy(), 538);
  }
}
