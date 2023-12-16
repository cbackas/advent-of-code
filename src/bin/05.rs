use itertools::Itertools;

advent_of_code::solution!(5);

#[derive(Debug, Clone, Copy)]
struct Map {
    source: Range,
    destination: Range,
    difference: i64,
}

#[derive(Debug, Clone, Copy)]
struct Range {
    start: u64,
    stop: u64,
}

fn map_map(line: &str) -> Map {
    // read the map lines, they have 3 numbers per line
    // destination range start, source range start, length of ranges
    let map_split = line
        .split_whitespace()
        .filter_map(|n| n.parse::<u64>().ok())
        .collect_vec();

    let source_start = map_split[1];
    let dest_start = map_split[0];
    let difference = dest_start as i64 - source_start as i64;

    Map {
        source: Range {
            start: source_start,
            stop: source_start + map_split[2],
        },
        destination: Range {
            start: dest_start,
            stop: dest_start + map_split[2],
        },
        difference,
    }
}

fn process_map_block<'a, I>(lines: &mut I) -> Vec<Map>
where
    I: Iterator<Item = &'a str>,
{
    lines
        .by_ref()
        .skip_while(|line| line.contains("map:"))
        .take_while(|line| !line.is_empty())
        .map(map_map)
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
                    if seed >= map.source.start && seed < map.source.stop {
                        seed = map.destination.start + (seed - map.source.start);
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

fn find_overlap(my_range: &Range, map_range: &Range) -> (Vec<Range>, Option<Range>) {
    let intersects = my_range.start >= map_range.start && my_range.start < map_range.stop
        || my_range.start <= map_range.start && my_range.stop > map_range.start;

    let intersection: Option<Range> = if intersects {
        let start = std::cmp::max(my_range.start, map_range.start);
        let stop = std::cmp::min(my_range.stop, map_range.stop);

        Some(Range { start, stop })
    } else {
        None
    };

    match intersection {
        Some(intersection) => {
            let mut remainder = vec![];
            if my_range.start < intersection.start {
                remainder.push(Range {
                    start: my_range.start,
                    stop: intersection.stop,
                })
            }
            if my_range.stop > intersection.stop {
                remainder.push(Range {
                    start: intersection.stop,
                    stop: my_range.stop,
                })
            }
            (remainder, Some(intersection))
        }
        None => (vec![my_range.clone()], None),
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let seeds = lines.next().unwrap_or("");
    let seeds = seeds
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect_vec();
    let mut seeds: Vec<Range> = seeds
        .iter()
        .tuples()
        .map(|(&start, &end)| Range {
            start,
            stop: start + end,
        })
        .collect();
    seeds.sort_by_key(|r| r.start);

    let mut lines = lines.rev();
    let mut map_blocks: Vec<Vec<Map>> = Vec::new();
    while let Some(_) = lines.next() {
        let maps = lines
            .by_ref()
            .skip_while(|line| line.is_empty())
            .take_while(|line| !line.contains("map:"))
            .map(map_map)
            .collect_vec();

        if maps.is_empty() {
            break;
        }

        map_blocks.push(maps);
    }
    map_blocks.reverse(); // we processed the map blocks in reverse so flip em back
    let min_range = map_blocks
        .into_iter()
        .fold(seeds, |seeds, seed_mappers| {
            let (unchanged_seeds, changed_seeds) = seed_mappers.into_iter().fold(
                (seeds, vec![]),
                |(mut unchanged_seeds, mut changed_seeds), seed_mapper| {
                    unchanged_seeds = unchanged_seeds
                        .into_iter()
                        .flat_map(|seed| {
                            let (remainders, intersection) =
                                find_overlap(&seed, &seed_mapper.source);

                            if let Some(intersection) = intersection {
                                changed_seeds.push(Range {
                                    start: (intersection.start as i64 + seed_mapper.difference)
                                        as u64,
                                    stop: (intersection.stop as i64 + seed_mapper.difference)
                                        as u64,
                                })
                            }
                            remainders
                        })
                        .collect();

                    (unchanged_seeds, changed_seeds)
                },
            );

            let concat = [unchanged_seeds, changed_seeds].concat();
            concat
        })
        .into_iter()
        .min_by(|&r, &o| r.start.cmp(&o.start));

    Some(min_range.unwrap().start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
