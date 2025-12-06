use std::cmp::max;
use std::ops::RangeInclusive;

advent_of_code::solution!(5);

pub fn parse_input(input: &str) -> (Vec<RangeInclusive<u64>>, String) {
    let (fresh_ranges, available_ingredients) = input.split_once("\n\n").unwrap();

    let fresh_ranges = fresh_ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();

            start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap()
        })
        .collect::<Vec<_>>();

    (fresh_ranges, available_ingredients.to_string())
}

pub fn part_one(input: &str) -> Option<usize> {
    let (fresh_ranges, available_ingredients) = parse_input(input);

    Some(
        available_ingredients
            .lines()
            .map(|ingredient| ingredient.parse::<u64>().unwrap())
            .filter(|ingredient| fresh_ranges.iter().any(|range| range.contains(ingredient)))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut fresh_ranges, _) = parse_input(input);

    fresh_ranges.sort_by_key(|range| *range.start());

    let mut current_highest_value = *fresh_ranges[0].end();
    let mut number_of_fresh_ids = fresh_ranges[0].end() - fresh_ranges[0].start() + 1;

    for range in fresh_ranges {
        // If the range ends after the highest known fresh element, the range is irrelevant
        if *range.end() <= current_highest_value {
            continue;
        }

        // Determine which part of this range is not yet processed
        let range_start = max(*range.start(), current_highest_value + 1);
        let non_processed_range_part = range_start..=*range.end();

        number_of_fresh_ids +=
            non_processed_range_part.end() - non_processed_range_part.start() + 1;
        current_highest_value = *range.end();
    }

    Some(number_of_fresh_ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
