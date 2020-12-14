use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const FLOOR: char = '.';
const OCCUPIED_SEAT: char = '#';
const EMPTY_SEAT: char = 'L';

fn read_seat_layout() -> Vec<Vec<char>> {
  let file = File::open("./inputs/day_11.txt").unwrap();

  let br = BufReader::new(file);
  let mut seat_layout: Vec<Vec<char>> = Vec::new();

  for line in br.lines() {
    let line = line.unwrap();
    seat_layout.push(line.chars().collect::<Vec<char>>());
  }

  seat_layout
}

fn is_closest_top_left_seat_occupied(
  seat_layout: &Vec<Vec<char>>,
  line_i: usize,
  col_i: usize,
) -> bool {
  let mut next_line_i: isize = line_i as isize - 1;
  let mut next_col_i: isize = col_i as isize - 1;

  while next_line_i >= 0 && next_col_i >= 0 {
    let c = seat_layout[next_line_i as usize][next_col_i as usize];
    if c == OCCUPIED_SEAT {
      return true;
    }
    if c == EMPTY_SEAT {
      return false;
    }

    next_line_i -= 1;
    next_col_i -= 1;
  }

  false
}

fn is_closest_top_seat_occupied(seat_layout: &Vec<Vec<char>>, line_i: usize, col_i: usize) -> bool {
  let mut next_line_i: isize = line_i as isize - 1;

  while next_line_i >= 0 {
    let c = seat_layout[next_line_i as usize][col_i];
    if c == OCCUPIED_SEAT {
      return true;
    }
    if c == EMPTY_SEAT {
      return false;
    }

    next_line_i -= 1;
  }

  false
}

fn is_closest_top_right_seat_occupied(
  seat_layout: &Vec<Vec<char>>,
  line_i: usize,
  col_i: usize,
) -> bool {
  let mut next_line_i: isize = line_i as isize - 1;
  let mut next_col_i = col_i + 1;

  while next_line_i >= 0 && next_col_i < seat_layout[line_i].len() {
    let c = seat_layout[next_line_i as usize][next_col_i];
    if c == OCCUPIED_SEAT {
      return true;
    }
    if c == EMPTY_SEAT {
      return false;
    }

    next_line_i -= 1;
    next_col_i += 1;
  }

  false
}

fn is_closest_left_seat_occupied(
  seat_layout: &Vec<Vec<char>>,
  line_i: usize,
  col_i: usize,
) -> bool {
  let mut next_col_i: isize = col_i as isize - 1;

  while next_col_i >= 0 {
    let c = seat_layout[line_i][next_col_i as usize];
    if c == OCCUPIED_SEAT {
      return true;
    }
    if c == EMPTY_SEAT {
      return false;
    }

    next_col_i -= 1;
  }

  false
}

fn is_closest_right_seat_occupied(
  seat_layout: &Vec<Vec<char>>,
  line_i: usize,
  col_i: usize,
) -> bool {
  let mut next_col_i = col_i + 1;

  while next_col_i < seat_layout[line_i].len() {
    let c = seat_layout[line_i][next_col_i];
    if c == OCCUPIED_SEAT {
      return true;
    }
    if c == EMPTY_SEAT {
      return false;
    }

    next_col_i += 1;
  }

  false
}

fn is_closest_bottom_left_seat_occupied(
  seat_layout: &Vec<Vec<char>>,
  line_i: usize,
  col_i: usize,
) -> bool {
  let mut next_line_i = line_i + 1;
  let mut next_col_i: isize = col_i as isize - 1;

  while next_line_i < seat_layout.len() && next_col_i >= 0 {
    let c = seat_layout[next_line_i][next_col_i as usize];
    if c == OCCUPIED_SEAT {
      return true;
    }
    if c == EMPTY_SEAT {
      return false;
    }

    next_line_i += 1;
    next_col_i -= 1;
  }

  false
}

fn is_closest_bottom_seat_occupied(
  seat_layout: &Vec<Vec<char>>,
  line_i: usize,
  col_i: usize,
) -> bool {
  let mut next_line_i = line_i + 1;

  while next_line_i < seat_layout.len() {
    let c = seat_layout[next_line_i][col_i];
    if c == OCCUPIED_SEAT {
      return true;
    }
    if c == EMPTY_SEAT {
      return false;
    }

    next_line_i += 1;
  }

  false
}

fn is_closest_bottom_right_seat_occupied(
  seat_layout: &Vec<Vec<char>>,
  line_i: usize,
  col_i: usize,
) -> bool {
  let mut next_line_i = line_i + 1;
  let mut next_col_i = col_i + 1;

  while next_line_i < seat_layout.len() && next_col_i < seat_layout[line_i].len() {
    let c = seat_layout[next_line_i][next_col_i];
    if c == OCCUPIED_SEAT {
      return true;
    }
    if c == EMPTY_SEAT {
      return false;
    }

    next_line_i += 1;
    next_col_i += 1;
  }

  false
}

fn count_occupied_adjacent_seats(
  seat_layout: &Vec<Vec<char>>,
  line_i: usize,
  col_i: usize,
) -> usize {
  let mut count = 0;

  // top line
  if is_closest_top_left_seat_occupied(&seat_layout, line_i, col_i) {
    count += 1;
  }
  if is_closest_top_seat_occupied(&seat_layout, line_i, col_i) {
    count += 1;
  }
  if is_closest_top_right_seat_occupied(&seat_layout, line_i, col_i) {
    count += 1;
  }

  // same line
  if is_closest_left_seat_occupied(&seat_layout, line_i, col_i) {
    count += 1;
  }
  if is_closest_right_seat_occupied(&seat_layout, line_i, col_i) {
    count += 1;
  }

  // bottom line
  if is_closest_bottom_left_seat_occupied(&seat_layout, line_i, col_i) {
    count += 1;
  }
  if is_closest_bottom_seat_occupied(&seat_layout, line_i, col_i) {
    count += 1;
  }
  if is_closest_bottom_right_seat_occupied(&seat_layout, line_i, col_i) {
    count += 1;
  }

  count
}

fn apply_rules_to_seat_layout(seat_layout: &Vec<Vec<char>>) -> Vec<Vec<char>> {
  let mut result_seat_layout: Vec<Vec<char>> = vec![];

  for line_i in 0..seat_layout.len() {
    let mut seat_layout_line: Vec<char> = vec![];

    for col_i in 0..seat_layout[line_i].len() {
      if seat_layout[line_i][col_i] == FLOOR {
        seat_layout_line.push(FLOOR);
        continue;
      }

      let occupied_adjacent_seats_count =
        count_occupied_adjacent_seats(&seat_layout, line_i, col_i);

      if seat_layout[line_i][col_i] == OCCUPIED_SEAT {
        if occupied_adjacent_seats_count >= 5 {
          seat_layout_line.push(EMPTY_SEAT);
        } else {
          seat_layout_line.push(OCCUPIED_SEAT);
        }
        continue;
      }

      if seat_layout[line_i][col_i] == EMPTY_SEAT {
        if occupied_adjacent_seats_count == 0 {
          seat_layout_line.push(OCCUPIED_SEAT);
        } else {
          seat_layout_line.push(EMPTY_SEAT);
        }
        continue;
      }

      panic!(
        "unknown char in seat layout: {:?}",
        seat_layout[line_i][col_i]
      );
    }

    result_seat_layout.push(seat_layout_line);
  }

  result_seat_layout
}

fn has_seat_layout_changed(
  old_seat_layout: &Vec<Vec<char>>,
  new_seat_layout: &Vec<Vec<char>>,
) -> bool {
  for line_i in 0..old_seat_layout.len() {
    for col_i in 0..old_seat_layout[line_i].len() {
      if old_seat_layout[line_i][col_i] != new_seat_layout[line_i][col_i] {
        return true;
      }
    }
  }

  false
}

fn count_occupied_seats(seat_layout: &Vec<Vec<char>>) -> usize {
  let mut count = 0;

  for line_i in 0..seat_layout.len() {
    for col_i in 0..seat_layout[line_i].len() {
      if seat_layout[line_i][col_i] == OCCUPIED_SEAT {
        count += 1;
      }
    }
  }

  count
}

pub fn seating_system() -> usize {
  let mut old_seat_layout = read_seat_layout();
  let mut new_seat_layout = apply_rules_to_seat_layout(&old_seat_layout);

  while has_seat_layout_changed(&old_seat_layout, &new_seat_layout) {
    old_seat_layout = new_seat_layout;
    new_seat_layout = apply_rules_to_seat_layout(&old_seat_layout);
  }

  count_occupied_seats(&new_seat_layout)
}
