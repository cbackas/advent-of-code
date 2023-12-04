use itertools::Itertools;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line| line.split(": ").map(|s| s.to_string()))
        .map(|split_line| {
            let card_sets = split_line.skip(1).next().unwrap();
            let card_sets = &card_sets.split(" | ").map(|s| s.to_string());
            card_sets.clone().collect_vec()
        })
        .map(|card_sets| {
            let card_sets = card_sets.iter().map(|set| {
                let cards = set.trim();
                let cards = cards
                    .split(" ")
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse::<u32>().unwrap());

                cards.collect_vec()
            });

            card_sets.collect_vec()
        })
        .map(|card_sets| {
            let set_1 = &card_sets[0];
            let set_2 = &card_sets[1];

            let mut matches = 0;

            for number in set_1 {
                if set_2.contains(number) {
                    matches += 1;
                }
            }

            matches
        })
        .map(|matches| {
            if matches == 0 {
                0
            } else {
                let base: u32 = 2;
                1 * base.pow(matches - 1)
            }
        })
        .sum();

    Some(result)
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