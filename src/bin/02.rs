use std::{cmp::Ordering, collections::HashSet, fmt::write};

advent_of_code::solution!(2);

fn parse_reports(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|cell| cell.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = parse_reports(input);

    let safe_reports_count = reports
        .iter()
        .filter(|report| {
            levels_are_ascending_or_descending(report)
                && adjacent_levels_do_not_bounce_excessively(report)
        })
        .count();

    Some(safe_reports_count as u32)
}

fn adjacent_levels_do_not_bounce_excessively(report: &[u32]) -> bool {
    report
        .iter()
        .zip(report.iter().skip(1))
        .map(|(left, right)| {
            let diff = left.abs_diff(*right);
            diff >= 1 && diff <= 3
        })
        .all(|l| l)
}

fn levels_are_ascending_or_descending(report: &[u32]) -> bool {
    let unique_levels_counts = report.iter().copied().collect::<HashSet<_>>().len();

    if unique_levels_counts != report.len() {
        return false;
    }

    let interlevel_bounce_directions: Vec<Ordering> = report
        .iter()
        .zip(report.iter().skip(1))
        .map(|(left, right)| left.cmp(right))
        .collect();

    interlevel_bounce_directions
        .iter()
        .all(|ilbd| *ilbd == Ordering::Less)
        || interlevel_bounce_directions
            .iter()
            .all(|ilbd| *ilbd == Ordering::Greater)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = parse_reports(input);

    let interlevel_relationship_vectors: Vec<Vec<InterlevelRelationship>> = reports
        .iter()
        .map(|report| generate_interlevel_relationships(report)).collect();


    None
}

fn generate_interlevel_relationships(report: &[u32]) -> Vec<InterlevelRelationship> {
    report.iter().zip(report.iter().skip(1)).map(|(left, right)| {
        let direction = left.cmp(right);
        let bounce_height = left.abs_diff(*right);
        let is_safe_bounce_distance = bounce_height >= 1 && bounce_height <= 3;
        InterlevelRelationship::new(direction, is_safe_bounce_distance)
    }).collect()
}

struct InterlevelRelationship {
    direction: Ordering,
    is_safe_bounce_distance: bool,
}

impl InterlevelRelationship {
    pub fn new(direction: Ordering, excessive_bounce: bool) -> Self {
        Self {
            direction,
            is_safe_bounce_distance: excessive_bounce,
        }
    }

    pub fn equals(&self, other: &Self) -> bool {
        self.direction == other.direction && self.is_safe_bounce_distance == other.is_safe_bounce_distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
