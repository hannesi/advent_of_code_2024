use std::{cmp::Ordering, collections::HashSet};

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
    adjacent_levels_safe_bounce_indicators(report)
        .iter()
        .all(|l| *l)
}

fn adjacent_levels_safe_bounce_indicators(report: &[u32]) -> Vec<bool> {
    report
        .iter()
        .zip(report.iter().skip(1))
        .map(|(left, right)| {
            let diff = left.abs_diff(*right);
            diff >= 1 && diff <= 3
        })
        .collect()
}

fn levels_are_ascending_or_descending(report: &[u32]) -> bool {
    let unique_levels_counts = report.iter().copied().collect::<HashSet<_>>().len();

    if unique_levels_counts != report.len() {
        return false;
    }

    let interlevel_bounce_directions = interlevel_bounce_direction_array(report);

    interlevel_bounce_directions
        .iter()
        .all(|ilbd| *ilbd == Ordering::Less)
        || interlevel_bounce_directions
            .iter()
            .all(|ilbd| *ilbd == Ordering::Greater)
}

fn interlevel_bounce_direction_array(report: &[u32]) -> Vec<Ordering> {
    report
        .iter()
        .zip(report.iter().skip(1))
        .map(|(left, right)| left.cmp(right))
        .collect()
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = parse_reports(input);

    None
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
