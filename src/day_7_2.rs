use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn parse_line(line: &String) -> (String, Vec<(String, usize)>) {
  let parts: Vec<&str> = line.split(" bags contain ").collect();
  let container_color: String = parts[0].to_string();
  let mut contained_colors_counts: Vec<(String, usize)> = vec![];

  if parts[0] != "no other bags." {
    let re = Regex::new(r"(?P<n>[\d]+)\s(?P<color>[\w\s]+)\sbag").unwrap();
    let captures = re.captures_iter(parts[1]);
    for capture in captures {
      contained_colors_counts.push((
        String::from(&capture["color"]),
        capture["n"].parse::<usize>().unwrap(),
      ));
    }
  }

  return (container_color, contained_colors_counts);
}

fn read_contained_in() -> HashMap<String, Vec<(String, usize)>> {
  let file = File::open("./inputs/day_7.txt").unwrap();

  let br = BufReader::new(file);

  let mut contained_in: HashMap<String, Vec<(String, usize)>> = HashMap::new();

  for line in br.lines() {
    let line = line.unwrap();
    let (container_color, contained_colors_counts) = parse_line(&line);

    contained_in.insert(container_color, contained_colors_counts);
  }

  contained_in
}

fn count_internal_bags(
  start_color: &String,
  contained_in: &HashMap<String, Vec<(String, usize)>>,
) -> usize {
  let mut count = 0;
  let mut current_queue: VecDeque<(String, usize)> = contained_in
    .get(start_color)
    .unwrap()
    .clone()
    .into_iter()
    .collect();
  let mut next_queue: VecDeque<(String, usize)> = VecDeque::new();

  while current_queue.len() > 0 {
    let (current_color, current_count) = current_queue.pop_front().unwrap();
    count += current_count;

    match contained_in.get(&current_color) {
      Some(next_colors_counts) => {
        for (next_color, next_count) in next_colors_counts {
          next_queue.push_back((next_color.to_string(), next_count * current_count));
        }
      }
      None => {}
    }

    if current_queue.len() == 0 {
      current_queue.append(&mut next_queue);
    }
  }

  count
}

pub fn handy_haversacks() -> usize {
  let contained_in = read_contained_in();
  let color = String::from("shiny gold");

  let count = count_internal_bags(&color, &contained_in);

  return count;
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_count_internal_bags_1() {
    /*
      shiny gold bags contain 2 dark red bags.
      dark red bags contain 2 dark orange bags.
      dark orange bags contain 2 dark yellow bags.
      dark yellow bags contain 2 dark green bags.
      dark green bags contain 2 dark blue bags.
      dark blue bags contain 2 dark violet bags.
      dark violet bags contain no other bags.
    */

    let mut contained_in: super::HashMap<String, Vec<(String, usize)>> = super::HashMap::new();
    contained_in.insert(
      String::from("shiny gold"),
      vec![(String::from("dark red"), 2)],
    );
    contained_in.insert(
      String::from("dark red"),
      vec![(String::from("dark orange"), 2)],
    );
    contained_in.insert(
      String::from("dark orange"),
      vec![(String::from("dark yellow"), 2)],
    );
    contained_in.insert(
      String::from("dark yellow"),
      vec![(String::from("dark green"), 2)],
    );
    contained_in.insert(
      String::from("dark green"),
      vec![(String::from("dark blue"), 2)],
    );
    contained_in.insert(
      String::from("dark blue"),
      vec![(String::from("dark violet"), 2)],
    );
    contained_in.insert(String::from("dark violet"), vec![]);

    assert_eq!(
      super::count_internal_bags(&String::from("shiny gold"), &contained_in),
      126
    );
  }

  #[test]
  fn test_count_internal_bags_2() {
    /*
      light red bags contain 1 bright white bag, 2 muted yellow bags.
      dark orange bags contain 3 bright white bags, 4 muted yellow bags.
      bright white bags contain 1 shiny gold bag.
      muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
      shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
      dark olive bags contain 3 faded blue bags, 4 dotted black bags.
      vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
      faded blue bags contain no other bags.
      dotted black bags contain no other bags.
    */

    let mut contained_in: super::HashMap<String, Vec<(String, usize)>> = super::HashMap::new();
    contained_in.insert(
      String::from("light red"),
      vec![
        (String::from("bright white"), 1),
        (String::from("muted yellow"), 2),
      ],
    );
    contained_in.insert(
      String::from("dark orange"),
      vec![
        (String::from("bright white"), 3),
        (String::from("muted yellow"), 4),
      ],
    );
    contained_in.insert(
      String::from("bright white"),
      vec![(String::from("shiny gold"), 1)],
    );
    contained_in.insert(
      String::from("muted yellow"),
      vec![
        (String::from("shiny gold"), 2),
        (String::from("faded blue"), 9),
      ],
    );
    contained_in.insert(
      String::from("shiny gold"),
      vec![
        (String::from("dark olive"), 1),
        (String::from("vibrant plum"), 2),
      ],
    );
    contained_in.insert(
      String::from("dark olive"),
      vec![
        (String::from("faded blue"), 3),
        (String::from("dotted black"), 4),
      ],
    );
    contained_in.insert(
      String::from("vibrant plum"),
      vec![
        (String::from("faded blue"), 5),
        (String::from("dotted black"), 6),
      ],
    );
    contained_in.insert(String::from("faded blue"), vec![]);
    contained_in.insert(String::from("dotted black"), vec![]);

    assert_eq!(
      super::count_internal_bags(&String::from("shiny gold"), &contained_in),
      32
    );
  }

  #[test]
  fn test_parse_line() {
    let input_1 = String::from("dotted black bags contain no other bags.");
    let input_2 = String::from("bright white bags contain 1 shiny gold bag.");
    let input_3 = String::from("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.");

    let result_1 = (String::from("dotted black"), vec![]);
    let result_2 = (
      String::from("bright white"),
      vec![(String::from("shiny gold"), 1)],
    );
    let result_3 = (
      String::from("vibrant plum"),
      vec![
        (String::from("faded blue"), 5),
        (String::from("dotted black"), 6),
      ],
    );

    assert_eq!(super::parse_line(&input_1), result_1);
    assert_eq!(super::parse_line(&input_2), result_2);
    assert_eq!(super::parse_line(&input_3), result_3);
  }
}
