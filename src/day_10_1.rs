use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_ratings() -> Vec<usize> {
  let file = File::open("./inputs/day_10.txt").unwrap();

  let br = BufReader::new(file);
  let mut ratings: Vec<usize> = Vec::new();

  for line in br.lines() {
    let line = line.unwrap();
    let n = line.trim().parse().unwrap();
    ratings.push(n);
  }

  ratings
}

fn count_1_jolt_diff(ratings: &mut Vec<usize>) -> usize {
  ratings.sort();
  let mut count = 0;
  if ratings[0] == 1 {
    count += 1;
  }
  for i in 0..(ratings.len() - 1) {
    if ratings[i + 1] - ratings[i] == 1 {
      count += 1;
    }
  }

  count
}

fn count_3_jolt_diff(ratings: &mut Vec<usize>) -> usize {
  ratings.sort();
  let mut count = 1;
  if ratings[0] == 3 {
    count += 1;
  }
  for i in 0..(ratings.len() - 1) {
    if ratings[i + 1] - ratings[i] == 3 {
      count += 1;
    }
  }

  count
}

pub fn adapter_array() -> usize {
  let mut ratings = read_ratings();
  let count_diff_1 = count_1_jolt_diff(&mut ratings);
  let count_diff_3 = count_3_jolt_diff(&mut ratings);
  count_diff_1 * count_diff_3
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_count_1_jolt_diff() {
    let mut ratings_1 = vec![
      28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17,
      7, 9, 4, 2, 34, 10, 3,
    ];
    let mut ratings_2 = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

    assert_eq!(super::count_1_jolt_diff(&mut ratings_1), 22);
    assert_eq!(super::count_1_jolt_diff(&mut ratings_2), 7);
  }

  #[test]
  fn test_count_3_jolt_diff() {
    let mut ratings_1 = vec![
      28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17,
      7, 9, 4, 2, 34, 10, 3,
    ];
    let mut ratings_2 = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

    assert_eq!(super::count_3_jolt_diff(&mut ratings_1), 10);
    assert_eq!(super::count_3_jolt_diff(&mut ratings_2), 5);
  }
}
