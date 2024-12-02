use std::collections::HashSet;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let reports: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|cell| cell.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

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

    let next_levels_is_higher: Vec<bool> = report
        .iter()
        .zip(report.iter().skip(1))
        .map(|(left, right)| left.lt(right))
        .collect();

    next_levels_is_higher.iter().all(|l| *l) || next_levels_is_higher.iter().all(|l| !*l)
}

pub fn part_two(input: &str) -> Option<u32> {
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
