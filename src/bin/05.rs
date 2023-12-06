use itertools::Itertools;

advent_of_code::solution!(5);

#[derive(Debug)]
struct Map {
    source_start: u64,
    destination_start: u64,
    length: u64,
}

fn process_map_block<'a, I>(lines: &mut I) -> Vec<Map>
where
    I: Iterator<Item = &'a str>,
{
    lines
        .by_ref()
        .skip_while(|line| line.contains("map:"))
        .take_while(|line| !line.is_empty())
        .map(|line| {
            // read the map lines, they have 3 numbers per line
            // destination range start, source range start, length of ranges
            let map_split = line
                .split_whitespace()
                .filter_map(|n| n.parse::<u64>().ok())
                .collect_vec();

            Map {
                source_start: map_split[1],
                destination_start: map_split[0],
                length: map_split[2],
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    let seeds = lines.next().unwrap_or("");
    let mut seeds = seeds
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect_vec();

    while let Some(_) = lines.next() {
        let maps = process_map_block(&mut lines);

        let transformed = seeds
            .iter()
            .map(|seed| {
                let mut seed = *seed;
                for map in &maps {
                    let source_end: u64 = (map.source_start + map.length) as u64;
                    if seed >= map.source_start && seed < source_end {
                        seed = map.destination_start + (seed - map.source_start);
                        break;
                    }
                }

                seed
            })
            .collect_vec();

        seeds = transformed;
    }

    let min = seeds.iter().min();
    match min {
        Some(min) => Some(*min as u32),
        None => None,
    }
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

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
