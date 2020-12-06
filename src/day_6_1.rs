use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_yes_answers_set_of_groups() -> Vec<HashSet<char>> {
  let file = File::open("./inputs/day_6.txt").unwrap();

  let br = BufReader::new(file);

  let mut groups_yes_answers: Vec<HashSet<char>> = Vec::new();
  let mut current_group_yes_answers: HashSet<char> = HashSet::new();

  for line in br.lines() {
    let line = line.unwrap();

    if line == "" {
      groups_yes_answers.push(current_group_yes_answers);
      current_group_yes_answers = HashSet::new();
    } else {
      for c in line.chars() {
        current_group_yes_answers.insert(c);
      }
    }
  }
  groups_yes_answers.push(current_group_yes_answers);

  groups_yes_answers
}

pub fn custom_customs() -> usize {
  let groups_yes_answers = read_yes_answers_set_of_groups();
  let mut sum = 0;
  for group_yes_answers in groups_yes_answers {
    sum += group_yes_answers.len();
  }

  return sum;
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_input() {
    assert_eq!(super::custom_customs(), 6633);
  }
}
