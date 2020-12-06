use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct GroupCount {
  yes_count_by_answer: HashMap<char, usize>,
  n_members: usize,
}

fn read_yes_answers_count_of_groups() -> Vec<GroupCount> {
  let file = File::open("./inputs/day_6.txt").unwrap();

  let br = BufReader::new(file);

  let mut groups_yes_answers: Vec<GroupCount> = Vec::new();
  let mut current_group_yes_answers: GroupCount = GroupCount {
    yes_count_by_answer: HashMap::new(),
    n_members: 0,
  };

  for line in br.lines() {
    let line = line.unwrap();

    if line == "" {
      groups_yes_answers.push(current_group_yes_answers);
      current_group_yes_answers = GroupCount {
        yes_count_by_answer: HashMap::new(),
        n_members: 0,
      };
    } else {
      for c in line.chars() {
        if !current_group_yes_answers
          .yes_count_by_answer
          .contains_key(&c)
        {
          current_group_yes_answers.yes_count_by_answer.insert(c, 0);
        }

        let counter = current_group_yes_answers
          .yes_count_by_answer
          .insert(c, 0)
          .unwrap()
          + 1;
        current_group_yes_answers
          .yes_count_by_answer
          .insert(c, counter);
      }
      current_group_yes_answers.n_members += 1;
    }
  }
  groups_yes_answers.push(current_group_yes_answers);

  groups_yes_answers
}

pub fn custom_customs() -> usize {
  let groups_yes_answers = read_yes_answers_count_of_groups();

  let mut sum = 0;
  for group_yes_answers in groups_yes_answers {
    for value in group_yes_answers.yes_count_by_answer.values().cloned() {
      if value == group_yes_answers.n_members {
        sum += 1;
      }
    }
  }

  return sum;
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_input() {
    assert_eq!(super::custom_customs(), 3202);
  }
}
