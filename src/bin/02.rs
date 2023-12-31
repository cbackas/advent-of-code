use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    static LIMITS: &[(&str, u32)] = &[("red", 12), ("blue", 14), ("green", 13)];

    let result = input
        .lines()
        .map(|line| {
            let (game, sets) = line.split_once(":").unwrap();
            let game = game.replace("Game ", "").parse::<u32>().unwrap();

            let sets = sets.split(";");
            let mut impossible = false;
            let hands = sets
                .map(|set| {
                    let set = set.trim();
                    let cubes = set.split(",").map(|cube| cube.trim()).collect_vec();

                    cubes
                })
                .collect_vec();

            let mut i = 0;
            while i < hands.len() {
                let hand = &hands[i];
                let mut j = 0;
                while j < hand.len() {
                    let cube = hand[j];
                    let (number, color) = cube.split_once(" ").unwrap();
                    let number = number.parse::<u32>().unwrap();

                    let (_, limit) = LIMITS.iter().find(|(color2, _)| color == *color2).unwrap();

                    if number > *limit {
                        impossible = true;
                        break;
                    }

                    j += 1;
                }

                if impossible {
                    break;
                }

                i += 1;
            }

            if impossible {
                0
            } else {
                game
            }
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line| {
            let (_game, sets) = line.split_once(":").unwrap();

            let sets = sets.split(";");
            let hands = sets
                .map(|set| {
                    let set = set.trim();
                    let cubes = set.split(",").map(|cube| cube.trim()).collect_vec();

                    cubes
                })
                .collect_vec();

            let mut max_reds = 0;
            let mut max_blues = 0;
            let mut max_greens = 0;

            let mut i = 0;
            while i < hands.len() {
                let hand = &hands[i];
                let mut j = 0;
                while j < hand.len() {
                    let cube = hand[j];
                    let (number, color) = cube.split_once(" ").unwrap();
                    let number = number.parse::<u32>().unwrap();

                    match color {
                        "red" => {
                            if number > max_reds {
                                max_reds = number;
                            }
                        }
                        "blue" => {
                            if number > max_blues {
                                max_blues = number;
                            }
                        }
                        "green" => {
                            if number > max_greens {
                                max_greens = number;
                            }
                        }
                        _ => unreachable!(),
                    }

                    j += 1;
                }

                i += 1;
            }

            let power = max_reds * max_blues * max_greens;

            power
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
