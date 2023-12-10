use itertools::Itertools;

advent_of_code::solution!(9);

fn extrapolate_next_item(vecs: Vec<&Vec<i32>>) -> i32 {
    let mut running_number = 0;

    let mut i = 0;
    loop {
        if let Some(next_thing) = vecs.get(i + 1) {
            let next_last_number = next_thing.last().unwrap();

            running_number += next_last_number;
        } else {
            break;
        }
        i += 1;
    }

    running_number
}

fn expand_number_vec(mut vecs: Vec<Vec<i32>>, current_index: usize) -> Vec<Vec<i32>> {
    let current_thing = &vecs[current_index];

    let mut new_array: Vec<i32> = Vec::new();
    let mut i = 0;
    while i < current_thing.len() - 1 {
        let j = i + 1;

        let curr_number = current_thing[i];
        let next_number = current_thing[j];

        let difference = next_number - curr_number;

        new_array.push(difference);

        i += 1;
    }

    vecs.push(new_array.clone());

    let zero_count = new_array.iter().filter(|&&x| x == 0).count();
    if zero_count == new_array.len() {
        return vecs;
    }

    expand_number_vec(vecs, current_index + 1)
}

pub fn part_one(input: &str) -> Option<i32> {
    let number_vecs = input
        .lines()
        .map(|line| line.split_whitespace().collect_vec())
        .map(|vec| vec.iter().map(|s| s.parse::<i32>().unwrap()).collect_vec())
        .map(|vec| {
            let expanded_vector = expand_number_vec(vec![vec], 0);
            let expanded_vector = expanded_vector.iter().rev().collect_vec();
            extrapolate_next_item(expanded_vector)
        })
        .sum();

    Some(number_vecs)
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
