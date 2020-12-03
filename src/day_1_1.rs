use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_entries() -> Vec<i32> {
  let file = File::open("./inputs/day_1.txt").unwrap();

  let br = BufReader::new(file);
  let mut entries: Vec<i32> = Vec::new();

  for line in br.lines() {
    let line = line.unwrap();
    let n = line.trim().parse().unwrap();
    entries.push(n);
  }

  entries
}

pub fn report_repair() -> i32 {
  let entries = read_entries();

  let set: HashSet<i32> = entries.iter().cloned().collect();

  for entry in entries {
    let diff = 2020 - entry;
    if set.contains(&diff) {
      println!("{:} + {:}", entry, diff);
      return entry * diff;
    }
  }

  println!("Not found");
  return 0;
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_input() {
    assert_eq!(super::report_repair(), 877971);
  }
}
