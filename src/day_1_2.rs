use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_entries() -> Vec<i32> {
  let file = File::open("./inputs/report_repair.txt").unwrap();

  let br = BufReader::new(file);
  let mut entries: Vec<i32> = Vec::new();

  for line in br.lines() {
    let line = line.unwrap();
    let n = line.trim().parse().unwrap();
    entries.push(n);
  }

  entries
}

pub fn report_repair_2() -> i32 {
  let entries = read_entries();

  let mut map = HashMap::new();

  for i in 0..entries.len() {
    for j in (i + 1)..entries.len() {
      map.insert(entries[i] + entries[j], [entries[i], entries[j]]);
    }
  }

  for entry in entries {
    let diff = 2020 - entry;
    if map.contains_key(&diff) {
      let values = map.get(&diff).unwrap();
      println!("{:} + {:} + {:}", entry, values[0], values[1]);
      return entry * values[0] * values[1];
    }
  }

  println!("Not found");
  return 0;
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_input() {
    assert_eq!(super::report_repair_2(), 203481432);
  }
}
