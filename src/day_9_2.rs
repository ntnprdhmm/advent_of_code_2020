use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_entries() -> Vec<usize> {
  let file = File::open("./inputs/day_9.txt").unwrap();

  let br = BufReader::new(file);
  let mut entries: Vec<usize> = Vec::new();

  for line in br.lines() {
    let line = line.unwrap();
    let n = line.trim().parse().unwrap();
    entries.push(n);
  }

  entries
}

fn is_number_valid(
  options_counts: &HashMap<usize, usize>,
  numbers: &Vec<usize>,
  i_number_to_validate: usize,
  options_size: usize,
) -> bool {
  for i_number_to_subtract in (i_number_to_validate - options_size)..i_number_to_validate {
    if numbers[i_number_to_validate] > numbers[i_number_to_subtract] {
      let remaining = numbers[i_number_to_validate] - numbers[i_number_to_subtract];
      let remaining_count_option = options_counts.get(&remaining);

      match remaining_count_option {
        Some(remaining_count) => {
          if remaining == numbers[i_number_to_subtract] && remaining_count >= &2 {
            return true;
          }
          if remaining != numbers[i_number_to_subtract] && remaining_count >= &1 {
            return true;
          }
        }
        None => {}
      }
    }
  }

  false
}

fn find_encryption_weakness(numbers: &Vec<usize>, expected_sum: usize) -> usize {
  for set_size in 2..numbers.len() {
    let mut sum = 0;
    for i in 0..set_size {
      sum += numbers[i];
    }

    if sum == expected_sum {
      let sub = &numbers[0..set_size];
      return sub.iter().min().unwrap() + sub.iter().max().unwrap();
    }

    for i in 1..(numbers.len() - set_size) {
      sum -= numbers[i - 1];
      sum += numbers[i + set_size - 1];

      if sum == expected_sum {
        let sub = &numbers[i..(i + set_size - 1)];
        return sub.iter().min().unwrap() + sub.iter().max().unwrap();
      }
    }
  }

  0
}

fn find_first_wrong_number(numbers: &Vec<usize>, options_size: usize) -> usize {
  let mut options_counts: HashMap<usize, usize> = HashMap::new();
  for i in 0..options_size {
    if options_counts.contains_key(&numbers[i]) {
      options_counts.insert(
        numbers[i].clone(),
        options_counts.get(&numbers[i]).unwrap() + 1,
      );
    } else {
      options_counts.insert(numbers[i].clone(), 1);
    }
  }

  for i_number_to_validate in options_size..numbers.len() {
    let is_valid = is_number_valid(
      &options_counts,
      &numbers,
      i_number_to_validate.clone(),
      options_size.clone(),
    );

    if !is_valid {
      return numbers[i_number_to_validate];
    }

    // remove oldest option
    if options_counts
      .get(&numbers[i_number_to_validate - options_size])
      .unwrap()
      > &1
    {
      options_counts.insert(
        numbers[i_number_to_validate],
        options_counts
          .get(&numbers[i_number_to_validate - options_size])
          .unwrap()
          - 1,
      );
    } else {
      options_counts.remove(&numbers[i_number_to_validate - options_size]);
    }

    // add next option
    if options_counts.contains_key(&numbers[i_number_to_validate]) {
      options_counts.insert(
        numbers[i_number_to_validate].clone(),
        options_counts.get(&numbers[i_number_to_validate]).unwrap() + 1,
      );
    } else {
      options_counts.insert(numbers[i_number_to_validate].clone(), 1);
    }
  }

  0
}

pub fn encoding_error() -> usize {
  let numbers = read_entries();
  let wrong_number = find_first_wrong_number(&numbers, 25);
  find_encryption_weakness(&numbers, wrong_number)
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_encoding_error() {
    assert_eq!(super::encoding_error(), 13935797);
  }

  #[test]
  fn test_find_encryption_weakness() {
    let numbers: Vec<usize> = vec![
      35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];
    assert_eq!(super::find_encryption_weakness(&numbers, 127), 62);
  }

  #[test]
  fn test_find_first_wrong_number() {
    let numbers: Vec<usize> = vec![
      35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];

    assert_eq!(super::find_first_wrong_number(&numbers, 5), 127);
  }
}
