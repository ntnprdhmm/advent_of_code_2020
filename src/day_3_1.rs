use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_entries() -> Vec<Vec<bool>> {
  let file = File::open("./inputs/day_3.txt").unwrap();

  let br = BufReader::new(file);

  // true means it's a tree, false it's empty
  let mut map: Vec<Vec<bool>> = Vec::new();

  for line in br.lines() {
    let mut map_line: Vec<bool> = Vec::new();
    let line = line.unwrap();

    for c in line.trim().chars() {
      let is_tree = c == '#';
      map_line.push(is_tree);
    }

    map.push(map_line);
  }

  map
}

pub fn toboggan_trajectory() -> usize {
  let map = read_entries();

  let mut trees_counter: usize = 0;

  let height = map.len();
  let width = map[0].len();
  let mut x = 0;

  for y in 0..(height - 1) {
    x = (x + 3) % width;

    if map[y + 1][x] == true {
      trees_counter += 1;
    }
  }

  return trees_counter;
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_input() {
    assert_eq!(super::toboggan_trajectory(), 252);
  }
}
