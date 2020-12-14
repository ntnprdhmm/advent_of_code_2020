use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct Instruction {
  action: char,
  value: usize,
}

fn read_instructions() -> Vec<Instruction> {
  let file = File::open("./inputs/day_12.txt").unwrap();

  let br = BufReader::new(file);
  let mut instructions: Vec<Instruction> = Vec::new();

  for line in br.lines() {
    let line = line.unwrap();
    let (action_str, value_str) = line.split_at(1);

    let instruction = Instruction {
      action: action_str.chars().nth(0).unwrap(),
      value: value_str.parse::<usize>().unwrap(),
    };

    instructions.push(instruction);
  }

  instructions
}

fn compute_new_waypoint((x, y): (isize, isize), degrees: isize) -> (isize, isize) {
  let mut d = degrees;
  if degrees < 0 {
    d = degrees + 360;
  }

  if d == 90 {
    return (-y, x);
  }

  if d == 180 {
    return (-x, -y);
  }

  (y, -x)
}

fn process_instructions(instructions: &Vec<Instruction>) -> (isize, isize) {
  let mut x: isize = 0;
  let mut y: isize = 0;
  let mut w_x: isize = 1;
  let mut w_y: isize = 10;

  for instruction in instructions {
    if instruction.action == 'L' {
      let (new_w_x, new_w_y) = compute_new_waypoint((w_x, w_y), -(instruction.value as isize));
      w_x = new_w_x;
      w_y = new_w_y;
      continue;
    }
    if instruction.action == 'R' {
      let (new_w_x, new_w_y) = compute_new_waypoint((w_x, w_y), instruction.value as isize);
      w_x = new_w_x;
      w_y = new_w_y;
      continue;
    }

    if instruction.action == 'F' {
      x += w_x * instruction.value as isize;
      y += w_y * instruction.value as isize;
    }

    if instruction.action == 'N' {
      w_x += instruction.value as isize;
      continue;
    }
    if instruction.action == 'S' {
      w_x -= instruction.value as isize;
      continue;
    }
    if instruction.action == 'E' {
      w_y += instruction.value as isize;
      continue;
    }
    if instruction.action == 'W' {
      w_y -= instruction.value as isize;
      continue;
    }
  }

  (x, y)
}

pub fn rain_risk() -> usize {
  let instructions = read_instructions();
  let (x, y) = process_instructions(&instructions);
  return (x.abs() + y.abs()) as usize;
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_compute_new_waypoint() {
    assert_eq!(super::compute_new_waypoint((4, 10), 90), (-10, 4));
    assert_eq!(super::compute_new_waypoint((4, 10), -270), (-10, 4));
    assert_eq!(super::compute_new_waypoint((4, 10), 180), (-4, -10));
    assert_eq!(super::compute_new_waypoint((4, 10), -180), (-4, -10));
    assert_eq!(super::compute_new_waypoint((4, 10), 270), (10, -4));
    assert_eq!(super::compute_new_waypoint((4, 10), -90), (10, -4));
  }

  #[test]
  fn test_process_instructions() {
    let instructions = vec![
      super::Instruction {
        action: 'F',
        value: 10,
      },
      super::Instruction {
        action: 'N',
        value: 3,
      },
      super::Instruction {
        action: 'F',
        value: 7,
      },
      super::Instruction {
        action: 'R',
        value: 90,
      },
      super::Instruction {
        action: 'F',
        value: 11,
      },
    ];

    assert_eq!(super::process_instructions(&instructions), (-72, 214));
  }
}
