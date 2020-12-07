use regex::Regex;
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn parse_contained_in_line(line: &String) -> (String, Vec<String>) {
  let parts: Vec<&str> = line.split(" bags contain ").collect();
  let container_color: String = parts[0].to_string();
  let mut contained_colors: Vec<String> = vec![];

  if parts[0] != "no other bags." {
    let re = Regex::new(r"[\d]+\s(?P<color>[\w\s]+)\sbag").unwrap();
    let captures = re.captures_iter(parts[1]);
    for capture in captures {
      contained_colors.push(String::from(&capture["color"]));
    }
  }

  return (container_color, contained_colors);
}

fn read_contained_in() -> HashMap<String, Vec<String>> {
  let file = File::open("./inputs/day_7.txt").unwrap();

  let br = BufReader::new(file);

  let mut contained_in: HashMap<String, Vec<String>> = HashMap::new();

  for line in br.lines() {
    let line = line.unwrap();
    let (container_color, contained_colors) = parse_contained_in_line(&line);

    for contained_color in contained_colors {
      if !contained_in.contains_key(&contained_color) {
        contained_in.insert(contained_color.clone(), vec![]);
      }

      let mut containers = contained_in.get(&contained_color).unwrap().clone();
      containers.push(container_color.clone());
      contained_in.insert(contained_color.clone(), containers);
    }
  }

  contained_in
}

fn count_color_containers(
  color: &String,
  contained_in_map: &HashMap<String, Vec<String>>,
) -> usize {
  let color_containers = contained_in_map.get(color).unwrap();
  let mut queue: VecDeque<String> = color_containers.clone().into_iter().collect();
  let mut count = queue.len();
  let mut color_processed: HashSet<String> = HashSet::new();
  for color_container in color_containers {
    color_processed.insert(color_container.to_string());
  }

  while queue.len() > 0 {
    let current_color = queue.pop_front().unwrap();
    let containers_option = contained_in_map.get(&current_color);
    match containers_option {
      Some(containers) => {
        for container in containers {
          if !color_processed.contains(container) {
            count += 1;
            queue.push_back(container.to_string());
            color_processed.insert(container.to_string());
          }
        }
      }
      None => {}
    };
  }

  count
}

pub fn handy_haversacks() -> usize {
  let color = String::from("shiny gold");
  let contained_in_map = read_contained_in();

  println!("{:?}", contained_in_map);

  let count = count_color_containers(&color, &contained_in_map);

  return count;
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_handy_haversacks() {
    assert_eq!(super::handy_haversacks(), 278);
  }

  #[test]
  fn test_parse_contained_in_line() {
    let input_1 = String::from("dotted black bags contain no other bags.");
    let input_2 = String::from("bright white bags contain 1 shiny gold bag.");
    let input_3 = String::from("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.");

    let result_1 = (String::from("dotted black"), vec![]);
    let result_2 = (
      String::from("bright white"),
      vec![String::from("shiny gold")],
    );
    let result_3 = (
      String::from("vibrant plum"),
      vec![String::from("faded blue"), String::from("dotted black")],
    );

    assert_eq!(super::parse_contained_in_line(&input_1), result_1);
    assert_eq!(super::parse_contained_in_line(&input_2), result_2);
    assert_eq!(super::parse_contained_in_line(&input_3), result_3);
  }

  #[test]
  fn test_count_color_containers() {
    let color = String::from("shiny gold");

    let mut contained_in_map: super::HashMap<String, Vec<String>> = super::HashMap::new();
    contained_in_map.insert(
      String::from("bright white"),
      vec![String::from("light red"), String::from("dark orange")],
    );
    contained_in_map.insert(
      String::from("muted yellow"),
      vec![String::from("light red"), String::from("dark orange")],
    );
    contained_in_map.insert(
      String::from("shiny gold"),
      vec![String::from("bright white"), String::from("muted yellow")],
    );
    contained_in_map.insert(
      String::from("dotted black"),
      vec![String::from("dark olive"), String::from("vibrant plum")],
    );
    contained_in_map.insert(String::from("dark olive"), vec![String::from("shiny gold")]);
    contained_in_map.insert(
      String::from("vibrant plum"),
      vec![String::from("shiny gold")],
    );
    contained_in_map.insert(
      String::from("faded blue"),
      vec![
        String::from("dark olive"),
        String::from("muted yellow"),
        String::from("vibrant plum"),
      ],
    );

    assert_eq!(super::count_color_containers(&color, &contained_in_map), 4);
  }
}
