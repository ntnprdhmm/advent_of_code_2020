use std::collections::HashMap;
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

fn count_arrangements(ratings: Vec<usize>) -> usize {
  let mut memo: HashMap<usize, usize> = HashMap::new();
  memo.insert(0, 1);

  for r in ratings.clone() {
    let mut count = 0;

    if r > 2 {
      match memo.get(&(r - 3)) {
        Some(c) => count += c,
        None => {}
      }
    }
    if r > 1 {
      match memo.get(&(r - 2)) {
        Some(c) => count += c,
        None => {}
      }
    }
    match memo.get(&(r - 1)) {
      Some(c) => count += c,
      None => {}
    }

    memo.insert(r, count);
  }

  memo.get(&ratings[ratings.len() - 1]).unwrap().clone()
}

pub fn adapter_array() -> usize {
  let mut ratings = read_ratings();
  ratings.sort();

  let mut extended_ratings = vec![];
  extended_ratings.append(&mut ratings);
  extended_ratings.push(extended_ratings[extended_ratings.len() - 1] + 3);

  count_arrangements(extended_ratings)
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_count_arrangements() {
    let ratings_1 = vec![1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, 22];
    let ratings_2 = vec![
      1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35, 38, 39,
      42, 45, 46, 47, 48, 49, 52,
    ];

    assert_eq!(super::count_arrangements(ratings_1), 8);
    assert_eq!(super::count_arrangements(ratings_2), 19208);
  }
}
