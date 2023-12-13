use itertools::Itertools;
use memoize::memoize;
use rayon::{
    prelude::{IntoParallelRefIterator, ParallelIterator},
    str::ParallelString,
};

advent_of_code::solution!(12);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, Clone, Hash)]
struct ConditionRecord {
    conditions: Vec<Condition>,
    parity: Vec<usize>,
}

fn parse_condition_records(input: &str, part_two: bool) -> Vec<ConditionRecord> {
    input
        .par_lines()
        .map(|line| {
            let split = line.split(" ").collect_vec();

            let conditions: Vec<Condition> = split[0]
                .par_chars()
                .map(|c| match c {
                    '.' => Condition::Operational,
                    '#' => Condition::Damaged,
                    '?' => Condition::Unknown,
                    _ => unreachable!(),
                })
                .collect();

            let parity: Vec<usize> = split[1].split(',').map(|s| s.parse().unwrap()).collect();

            if !part_two {
                ConditionRecord { conditions, parity }
            } else {
                let mut new_conditions = conditions.clone();
                let mut new_parity = parity.clone();
                for _ in 0..4 {
                    new_conditions.push(Condition::Unknown);
                    new_conditions.extend(conditions.clone());
                    new_parity.extend(parity.clone());
                }

                ConditionRecord {
                    conditions: new_conditions,
                    parity: new_parity,
                }
            }
        })
        .collect()
}

#[memoize]
fn check_condition(conditions: Vec<Condition>, parity: Vec<usize>) -> usize {
    if conditions.len() == 0 && parity.len() != 0 {
        return 0;
    } else if conditions.len() == 0 && parity.len() == 0 {
        return 1;
    }

    let starts_operational = conditions.starts_with(&[Condition::Operational]);
    if starts_operational {
        let conditions = conditions
            .iter()
            .skip_while(|c| **c == Condition::Operational)
            .cloned()
            .collect();
        return check_condition(conditions, parity);
    }

    let starts_unknown = conditions.starts_with(&[Condition::Unknown]);
    if starts_unknown {
        let try_operational = {
            let mut conditions = conditions.clone();
            conditions[0] = Condition::Operational;
            check_condition(conditions.to_vec(), parity.clone())
        };

        let try_damaged = {
            let mut conditions = conditions.clone();
            conditions[0] = Condition::Damaged;
            check_condition(conditions.to_vec(), parity.clone())
        };

        return try_operational + try_damaged;
    }

    if parity.len() == 0 {
        return 0;
    } else if parity[0] > conditions.len() {
        return 0;
    }

    let slice = conditions[0..parity[0]].to_vec();
    let contains_operational = slice
        .par_iter()
        .filter(|c| **c == Condition::Operational)
        .count()
        != 0;
    if contains_operational {
        return 0;
    } else {
        let mut conditions = conditions
            .iter()
            .skip(slice.len())
            .map(|c| *c)
            .collect_vec();

        if conditions.len() != 0 {
            if conditions[0] == Condition::Operational {
                conditions.remove(0);
            } else if conditions[0] == Condition::Damaged {
                return 0;
            } else if conditions[0] == Condition::Unknown {
                conditions[0] = Condition::Operational;
            }
        }

        let mut parity = parity.clone();
        parity.remove(0);

        return check_condition(conditions, parity);
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let condition_records = parse_condition_records(input, false);

    let result: usize = condition_records
        .par_iter()
        .map(|record| check_condition(record.conditions.clone(), record.parity.clone()))
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let condition_records = parse_condition_records(input, true);

    let result: usize = condition_records
        .par_iter()
        .map(|record| check_condition(record.conditions.clone(), record.parity.clone()))
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
