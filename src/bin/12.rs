use itertools::Itertools;
use rayon::{
    prelude::{IntoParallelRefIterator, ParallelIterator},
    str::ParallelString,
};

advent_of_code::solution!(12);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, Clone)]
struct ConditionRecord {
    conditions: Vec<Condition>,
    parity: Vec<usize>,
}

fn parse_condition_records(input: &str) -> Vec<ConditionRecord> {
    input
        .par_lines()
        .map(|line| {
            let split = line.split(" ").collect_vec();

            let spring_conditions: Vec<Condition> = split[0]
                .par_chars()
                .map(|c| match c {
                    '.' => Condition::Operational,
                    '#' => Condition::Damaged,
                    '?' => Condition::Unknown,
                    _ => unreachable!(),
                })
                .collect();

            let parity: Vec<usize> = split[1].split(',').map(|s| s.parse().unwrap()).collect();

            ConditionRecord {
                conditions: spring_conditions,
                parity,
            }
        })
        .collect()
}

fn check_condition(
    mut conditions: Vec<Condition>,
    mut parity: Vec<usize>,
    curr_depth: usize,
) -> usize {
    if conditions.len() == 0 && parity.len() != 0 {
        return 0;
    } else if conditions.len() == 0 && parity.len() == 0 {
        // println!("{}valid records found", "_".repeat(curr_depth - 1),);
        return 1;
    }

    // let indent = "_".repeat(curr_depth);

    let starts_operational = conditions.starts_with(&[Condition::Operational]);
    if starts_operational {
        // println!("{}{:?} {:?} starts operational", indent, conditions, parity);
        conditions = conditions
            .iter()
            .skip_while(|c| **c == Condition::Operational)
            .cloned()
            .collect();
        return check_condition(conditions, parity, curr_depth + 1);
    }

    let starts_unknown = conditions.starts_with(&[Condition::Unknown]);
    if starts_unknown {
        // println!("{}{:?} {:?} starts unknown", indent, conditions, parity);
        let try_operational = {
            let conditions = &mut conditions;
            conditions[0] = Condition::Operational;
            check_condition(conditions.to_vec(), parity.clone(), curr_depth + 1)
        };

        let try_damaged = {
            let conditions = &mut conditions;
            conditions[0] = Condition::Damaged;
            check_condition(conditions.to_vec(), parity.clone(), curr_depth + 1)
        };

        return try_operational + try_damaged;
    }

    // if it gets to this point then it starts with damaged
    // println!("{}{:?} {:?} starts damaged", indent, conditions, parity);

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
        conditions = conditions
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
        parity.remove(0);
        return check_condition(conditions, parity, curr_depth + 1);
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let condition_records = parse_condition_records(input);

    let result: usize = condition_records
        .par_iter()
        .map(|record| check_condition(record.conditions.clone(), record.parity.clone(), 0))
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
        assert_eq!(result, Some(21));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
