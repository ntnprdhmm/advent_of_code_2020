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

pub fn count_trees(map: &Vec<Vec<bool>>, right_step: usize, bottom_step: usize) -> usize {
  let mut trees_counter: usize = 0;

  let height = map.len();
  let width = map[0].len();
  let mut x = 0;

  for y in (0..(height - 1)).step_by(bottom_step) {
    x = (x + right_step) % width;

    if map[y + bottom_step][x] == true {
      trees_counter += 1;
    }
  }

  return trees_counter;
}

pub fn toboggan_trajectory() -> usize {
  let map = read_entries();

  let right_1_bottom_1 = count_trees(&map, 1, 1);
  let right_3_bottom_1 = count_trees(&map, 3, 1);
  let right_5_bottom_1 = count_trees(&map, 5, 1);
  let right_7_bottom_1 = count_trees(&map, 7, 1);
  let right_1_bottom_2 = count_trees(&map, 1, 2);

  return right_1_bottom_1
    * right_3_bottom_1
    * right_5_bottom_1
    * right_7_bottom_1
    * right_1_bottom_2;
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_input() {
    assert_eq!(super::toboggan_trajectory(), 2608962048);
  }
}
