use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct PasswordInput {
  password: String,
  letter: char,
  index_1: usize,
  index_2: usize,
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
      index_1: min_occ_string.parse::<usize>().unwrap() - 1,
      index_2: max_occ_string.parse::<usize>().unwrap() - 1,
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
    let password_chars: Vec<char> = entry.password.chars().collect();
    let mut sub_counter = 0;

    if password_chars[entry.index_1] == entry.letter {
      sub_counter += 1;
    }

    if password_chars[entry.index_2] == entry.letter {
      sub_counter += 1;
    }

    if sub_counter == 1 {
      counter += 1;
    }
  }

  return counter;
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_input() {
    assert_eq!(super::password_philosophy(), 489);
  }
}
