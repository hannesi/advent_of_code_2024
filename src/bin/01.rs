use std::{collections::HashMap, fmt::write};

advent_of_code::solution!(1);

fn split_str_to_col_vectors(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (left, right): (Vec<(usize, u32)>, Vec<(usize, u32)>) = input
        .lines()
        .flat_map(|line| {
            line.split_whitespace()
                .map(|cell| cell.parse::<u32>().unwrap())
        })
        .enumerate()
        .partition(|&(i, _)| i % 2 == 0);

    let (left_u32, right_u32) = (left.into_iter().map(|(_, v)| v).collect::<Vec<u32>>(), right.into_iter().map(|(_, v)| v).collect::<Vec<u32>>());

    (left_u32, right_u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = split_str_to_col_vectors(input);

    left.sort();
    right.sort();

    let total_distance = left.iter().zip(right.iter()).map(|pair| pair.0.abs_diff(*pair.1)).sum::<u32>();

    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = split_str_to_col_vectors(input);

    let mut frequency_map: HashMap<u32, u32> = HashMap::new();

    for num in right {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    let similarity_score: u32 = left.iter().map(|v| v * frequency_map.get(v).unwrap_or(&0)).sum();

    Some(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
