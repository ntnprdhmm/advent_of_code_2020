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

fn compute_new_direction(current_direction: char, degrees: isize) -> char {
  let directions = ['n', 'e', 's', 'w'];
  let current_direction_i = directions
    .iter()
    .position(|&d| d == current_direction)
    .unwrap();

  let mut i: usize = 0;
  if degrees > 0 {
    i = (degrees as usize) / 90;
  } else {
    i = ((degrees + 360) as usize) / 90;
  }

  let next_direction_i = (current_direction_i + i) % 4;
  directions[next_direction_i]
}

fn process_instructions(instructions: &Vec<Instruction>) -> (isize, isize) {
  let mut x: isize = 0;
  let mut y: isize = 0;

  let mut direction: char = 'e';

  for instruction in instructions {
    if instruction.action == 'L' {
      direction = compute_new_direction(direction, -(instruction.value as isize));
      continue;
    }
    if instruction.action == 'R' {
      direction = compute_new_direction(direction, instruction.value as isize);
      continue;
    }

    if instruction.action == 'F' {
      if direction == 'e' {
        y += instruction.value as isize;
        continue;
      }
      if direction == 'w' {
        y -= instruction.value as isize;
        continue;
      }
      if direction == 'n' {
        x += instruction.value as isize;
        continue;
      }
      if direction == 's' {
        x -= instruction.value as isize;
        continue;
      }
    }

    if instruction.action == 'N' {
      x += instruction.value as isize;
      continue;
    }
    if instruction.action == 'S' {
      x -= instruction.value as isize;
      continue;
    }
    if instruction.action == 'E' {
      y += instruction.value as isize;
      continue;
    }
    if instruction.action == 'W' {
      y -= instruction.value as isize;
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
  fn test_compute_new_direction() {
    assert_eq!(super::compute_new_direction('n', 270), 'w');
    assert_eq!(super::compute_new_direction('n', -270), 'e');
    assert_eq!(super::compute_new_direction('n', -90), 'w');
    assert_eq!(super::compute_new_direction('e', 180), 'w');
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

    assert_eq!(super::process_instructions(&instructions), (-8, 17));
  }
}
