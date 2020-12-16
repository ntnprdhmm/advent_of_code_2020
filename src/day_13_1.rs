use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_timestamp() -> (usize, Vec<usize>) {
  let file = File::open("./inputs/day_13.txt").unwrap();

  let br = BufReader::new(file);

  let mut lines_str: Vec<String> = vec![];
  for line in br.lines() {
    let line = line.unwrap();
    lines_str.push(line);
  }

  let timestamp = lines_str[0].parse::<usize>().unwrap();

  let bus_timestamps: Vec<usize> = lines_str[1]
    .split(',')
    .collect::<Vec<&str>>()
    .into_iter()
    .filter(|v| v.to_string() != "x")
    .map(|v| v.parse::<usize>().unwrap())
    .collect();

  (timestamp, bus_timestamps)
}

fn caculate_next_bus_departure_timestamp(timestamp: usize, bus_timestamp: usize) -> usize {
  timestamp + bus_timestamp - (timestamp % bus_timestamp)
}

fn calculate_next_bus_id(timestamp: usize, bus_timestamps: Vec<usize>) -> usize {
  let mut min_bus_timestamp = usize::MAX;
  let mut min_bus_wait_timestamp = usize::MAX;

  for bus_timestamp in bus_timestamps {
    let next_bus_departure_timestamp =
      caculate_next_bus_departure_timestamp(timestamp, bus_timestamp);
    if next_bus_departure_timestamp < min_bus_wait_timestamp {
      min_bus_wait_timestamp = next_bus_departure_timestamp;
      min_bus_timestamp = bus_timestamp;
    }
  }

  min_bus_timestamp * (min_bus_wait_timestamp - timestamp)
}

pub fn shuttle_search() -> usize {
  let (timestamp, bus_timestamps) = read_timestamp();
  calculate_next_bus_id(timestamp, bus_timestamps)
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_caculate_next_bus_departure_timestamp() {
    assert_eq!(super::caculate_next_bus_departure_timestamp(939, 7), 945);
    assert_eq!(super::caculate_next_bus_departure_timestamp(939, 13), 949);
  }
}
