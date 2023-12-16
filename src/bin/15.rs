use itertools::Itertools;
use rayon::{prelude::ParallelIterator, str::ParallelString};

advent_of_code::solution!(15);

fn mutate_char(starting: u32, c: char) -> u32 {
    let mut number = starting;
    number += c as u32;
    number *= 17;
    number %= 256;

    number
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .par_split(',')
        .map(|group| group.trim())
        .map(|group| group.chars().fold(0 as u32, mutate_char))
        .sum();

    Some(result)
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: u32,
}

#[derive(Debug, Clone)]
struct LensBox {
    lenses: Vec<Lens>,
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input
        .split(',')
        .map(|group| group.trim())
        .fold(
            vec![LensBox { lenses: vec![] }; 256],
            |state, group: &str| {
                let mut state = state.clone();

                if group.contains('=') {
                    let split = group.split('=').collect_vec();

                    let label = split[0].to_string();
                    let lens = Lens {
                        label: label.clone(),
                        focal_length: split[1].parse::<u32>().unwrap(),
                    };

                    let box_num = label.chars().fold(0 as u32, mutate_char);
                    let lenses = &state[box_num as usize].lenses;

                    if let Some((index, _lens)) = lenses
                        .iter()
                        .enumerate()
                        .find(|(_index, lens)| lens.label == label)
                    {
                        state[box_num as usize].lenses[index] = lens;
                    } else {
                        state[box_num as usize].lenses.push(lens);
                    }
                } else {
                    let label = group.trim_matches('-').to_string();

                    let box_num = label.chars().fold(0 as u32, mutate_char);
                    let lenses = &state[box_num as usize].lenses;

                    if let Some((index, _lens)) = lenses
                        .iter()
                        .enumerate()
                        .find(|(_index, lens)| lens.label == label)
                    {
                        state[box_num as usize].lenses.remove(index);
                    }
                }

                state
            },
        )
        .iter()
        .enumerate()
        .map(|(box_index, lens_box)| {
            let bux_multiplier = 1 + box_index as u32;

            lens_box
                .lenses
                .iter()
                .enumerate()
                .map(|(lens_index, lens)| {
                    let lens_multiplier = 1 + lens_index as u32;

                    bux_multiplier * lens_multiplier * lens.focal_length
                })
                .sum::<u32>()
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));

        assert_eq!(result, Some(145));
    }
}
