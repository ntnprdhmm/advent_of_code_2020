use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_instructions() -> Vec<(String, isize)> {
  let file = File::open("./inputs/day_8.txt").unwrap();

  let br = BufReader::new(file);

  let mut instructions: Vec<(String, isize)> = vec![];

  for line in br.lines() {
    let line = line.unwrap();

    let line_parts: Vec<&str> = line.split(" ").collect();

    let operation: String = line_parts[0].to_string();
    let argument = line_parts[1].parse::<isize>().unwrap();

    instructions.push((operation, argument));
  }

  instructions
}

fn is_infinite_loop(instructions: &Vec<(String, isize)>) -> (bool, isize) {
  let mut accumulator: isize = 0;
  let mut current_instruction_index: usize = 0;
  let mut already_visited_instructions: HashSet<usize> = HashSet::new();

  while !already_visited_instructions.contains(&current_instruction_index) {
    already_visited_instructions.insert(current_instruction_index);

    if instructions[current_instruction_index].0 == "acc" {
      accumulator += instructions[current_instruction_index].1;
    }

    if instructions[current_instruction_index].0 == "nop"
      || instructions[current_instruction_index].0 == "acc"
    {
      current_instruction_index += 1;
      if current_instruction_index == instructions.len() {
        return (false, accumulator);
      }
      current_instruction_index = current_instruction_index % instructions.len();
    } else {
      let next_index_without_mod =
        current_instruction_index as isize + instructions[current_instruction_index].1;

      if next_index_without_mod >= 0 {
        if next_index_without_mod >= instructions.len() as isize {
          current_instruction_index = (next_index_without_mod as usize) % instructions.len();
          if current_instruction_index == 0 {
            return (false, accumulator);
          }
        } else {
          current_instruction_index = (next_index_without_mod as usize) % instructions.len();
        }
      } else {
        current_instruction_index = ((next_index_without_mod % instructions.len() as isize)
          + instructions.len() as isize) as usize;
      }
    }
  }

  (true, accumulator)
}

fn fix_instructions(instructions: &Vec<(String, isize)>) -> isize {
  for i in 0..instructions.len() {
    if instructions[i].0 == "nop" || instructions[i].0 == "jmp" {
      let mut instructions_clone = instructions.clone();
      if instructions_clone[i].0 == "nop" {
        instructions_clone[i].0 = String::from("jmp");
      } else {
        instructions_clone[i].0 = String::from("nop");
      }

      let (is_infinite, acc) = is_infinite_loop(&instructions_clone);
      if !is_infinite {
        return acc;
      }
    }
  }

  0
}

pub fn handheld_halting() -> isize {
  let broken_instructions = read_instructions();
  fix_instructions(&broken_instructions)
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_fix_instructions() {
    let mut instructions: Vec<(String, isize)> = vec![];
    instructions.push((String::from("nop"), 0));
    instructions.push((String::from("acc"), 1));
    instructions.push((String::from("jmp"), 4));
    instructions.push((String::from("acc"), 3));
    instructions.push((String::from("jmp"), -3));
    instructions.push((String::from("acc"), -99));
    instructions.push((String::from("acc"), 1));
    instructions.push((String::from("jmp"), -4));
    instructions.push((String::from("acc"), 6));

    assert_eq!(super::fix_instructions(&instructions), 8);
  }

  #[test]
  fn test_handhelf_halting() {
    assert_eq!(super::handheld_halting(), 1000);
  }
}
