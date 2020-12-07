use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_entries() -> Vec<Vec<char>> {
  let file = File::open("./inputs/day_5.txt").unwrap();

  let br = BufReader::new(file);
  let mut boarding_passes: Vec<Vec<char>> = Vec::new();

  for line in br.lines() {
    let line = line.unwrap();
    let letters: Vec<char> = line.chars().collect();
    boarding_passes.push(letters);
  }

  boarding_passes
}

fn compute_col(boarding_pass: &Vec<char>) -> usize {
  let mut x = 0;
  let mut y = 7;
  let mut half_size = 4;

  for i in 7..10 {
    if boarding_pass[i] == 'L' {
      y = y - half_size;
    } else {
      x = x + half_size;
    }

    half_size = half_size / 2;
  }

  return x;
}

fn compute_row(boarding_pass: &Vec<char>) -> usize {
  let mut x = 0;
  let mut y = 127;
  let mut half_size = 64;

  for i in 0..7 {
    if boarding_pass[i] == 'F' {
      y = y - half_size;
    } else {
      x = x + half_size;
    }

    half_size = half_size / 2;
  }

  return x;
}

fn compute_seat_id(boarding_pass: &Vec<char>) -> usize {
  let row = compute_row(&boarding_pass);
  let col = compute_col(&boarding_pass);
  return row * 8 + col;
}

pub fn binary_boarding() -> usize {
  let boarding_passes = read_entries();
  let mut seat_ids: Vec<usize> = vec![];
  for boarding_pass in boarding_passes {
    seat_ids.push(compute_seat_id(&boarding_pass));
  }

  seat_ids.sort();
  for i in 1..seat_ids.len() {
    if seat_ids[i] - seat_ids[i - 1] == 2 {
      return seat_ids[i] - 1;
    }
  }

  return 0;
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_compute_row() {
    let boarding_pass_1 = vec!['F', 'B', 'F', 'B', 'B', 'F', 'F', 'R', 'L', 'R'];
    let boarding_pass_2 = vec!['B', 'F', 'F', 'F', 'B', 'B', 'F', 'R', 'R', 'R'];
    let boarding_pass_3 = vec!['F', 'F', 'F', 'B', 'B', 'B', 'F', 'R', 'R', 'R'];
    let boarding_pass_4 = vec!['B', 'B', 'F', 'F', 'B', 'B', 'F', 'R', 'L', 'L'];

    assert_eq!(super::compute_row(&boarding_pass_1), 44);
    assert_eq!(super::compute_row(&boarding_pass_2), 70);
    assert_eq!(super::compute_row(&boarding_pass_3), 14);
    assert_eq!(super::compute_row(&boarding_pass_4), 102);
  }

  #[test]
  fn test_compute_col() {
    let boarding_pass_1 = vec!['F', 'B', 'F', 'B', 'B', 'F', 'F', 'R', 'L', 'R'];
    let boarding_pass_2 = vec!['B', 'F', 'F', 'F', 'B', 'B', 'F', 'R', 'R', 'R'];
    let boarding_pass_3 = vec!['F', 'F', 'F', 'B', 'B', 'B', 'F', 'R', 'R', 'R'];
    let boarding_pass_4 = vec!['B', 'B', 'F', 'F', 'B', 'B', 'F', 'R', 'L', 'L'];

    assert_eq!(super::compute_col(&boarding_pass_1), 5);
    assert_eq!(super::compute_col(&boarding_pass_2), 7);
    assert_eq!(super::compute_col(&boarding_pass_3), 7);
    assert_eq!(super::compute_col(&boarding_pass_4), 4);
  }

  #[test]
  fn test_compute_seat_id() {
    let boarding_pass_1 = vec!['F', 'B', 'F', 'B', 'B', 'F', 'F', 'R', 'L', 'R'];
    let boarding_pass_2 = vec!['B', 'F', 'F', 'F', 'B', 'B', 'F', 'R', 'R', 'R'];
    let boarding_pass_3 = vec!['F', 'F', 'F', 'B', 'B', 'B', 'F', 'R', 'R', 'R'];
    let boarding_pass_4 = vec!['B', 'B', 'F', 'F', 'B', 'B', 'F', 'R', 'L', 'L'];

    assert_eq!(super::compute_seat_id(&boarding_pass_1), 357);
    assert_eq!(super::compute_seat_id(&boarding_pass_2), 567);
    assert_eq!(super::compute_seat_id(&boarding_pass_3), 119);
    assert_eq!(super::compute_seat_id(&boarding_pass_4), 820);
  }
}
