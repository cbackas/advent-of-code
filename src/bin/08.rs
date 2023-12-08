use itertools::Itertools;

advent_of_code::solution!(8);

struct Map {
    key: String,
    left_value: String,
    right_value: String,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    let directions = lines.next().unwrap();
    let directions = directions.chars().collect_vec();

    let maps: Vec<Map> = lines
        .skip(1)
        .map(|line| {
            let mut parts = line.split(" = ");
            let key = parts.next().unwrap();
            let value = parts.next().unwrap();
            let value = value
                .trim_matches(|c| c == '(' || c == ')')
                .split(", ")
                .collect_vec();

            Map {
                key: key.to_string(),
                left_value: value[0].to_string(),
                right_value: value[1].to_string(),
            }
        })
        .collect();

    let mut current_position: &str = "AAA";
    let mut steps = 0;
    loop {
        let direction = directions[steps % directions.len()];

        let found = maps.iter().find(|map| map.key == current_position).unwrap();
        current_position = match direction {
            'L' => &found.left_value,
            'R' => &found.right_value,
            _ => panic!("Unknown direction"),
        };

        steps += 1;

        if current_position == "ZZZ" {
            break;
        }
    }

    Some(steps as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
