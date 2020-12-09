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

fn find_accumulator_value(instructions: &Vec<(String, isize)>) -> isize {
  let mut accumulator: isize = 0;
  let mut current_instruction_index: usize = 0;
  let mut already_visited_instructions: HashSet<usize> = HashSet::new();

  while !already_visited_instructions.contains(&current_instruction_index) {
    already_visited_instructions.insert(current_instruction_index);

    if instructions[current_instruction_index].0 == "nop" {
      current_instruction_index = (current_instruction_index + 1) % instructions.len();
    } else if instructions[current_instruction_index].0 == "jmp" {
      let next_index_without_mod =
        current_instruction_index as isize + instructions[current_instruction_index].1;

      if next_index_without_mod >= 0 {
        current_instruction_index = (next_index_without_mod as usize) % instructions.len();
      } else {
        current_instruction_index = ((next_index_without_mod % instructions.len() as isize)
          + instructions.len() as isize) as usize;
      }
    } else {
      accumulator += instructions[current_instruction_index].1;
      current_instruction_index = (current_instruction_index + 1) % instructions.len();
    }
  }

  accumulator
}

pub fn handheld_halting() -> isize {
  let instructions = read_instructions();
  find_accumulator_value(&instructions)
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_find_accumulator_value() {
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

    assert_eq!(super::find_accumulator_value(&instructions), 5);
  }
}
