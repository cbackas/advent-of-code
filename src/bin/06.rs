use itertools::Itertools;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect_vec();

    let times = lines[0]
        .split_whitespace()
        .filter_map(|n| n.parse::<f32>().ok())
        .collect_vec();
    let distances = lines[1]
        .split_whitespace()
        .filter_map(|n| n.parse::<f32>().ok())
        .collect_vec();

    let mut running = 1;

    let mut i = 0;
    while i < times.len() {
        let b = times[i];
        let c = distances[i];

        let discrim = ((b * b) - 4.0 * c).sqrt();
        let x1 = ((-b + discrim) / (-2.0)).floor() + 1.0;
        let x2 = ((-b - discrim) / (-2.0)).ceil() + 1.0;

        let result = (x1 - x2).abs() - 1.0;

        running *= result as u32;

        i += 1;
    }

    Some(running)
}

pub fn part_two(_input: &str) -> Option<u32> {
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
