use itertools::Itertools;
use std::collections::HashSet;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    part_one_(input, 1000)
}

fn parse_input(input: &str) -> Vec<(i64, i64, i64)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut line = line.split(',');
            (
                line.next().unwrap().parse().unwrap(),
                line.next().unwrap().parse().unwrap(),
                line.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

fn sort_by_distance(points: &[(i64, i64, i64)]) -> Vec<(usize, usize)> {
    points
        .iter()
        .enumerate()
        .flat_map(|(i1, (x1, y1, z1))| {
            points[i1 + 1..]
                .iter()
                .enumerate()
                .map(move |(i2, (x2, y2, z2))| {
                    (
                        (i1, i1 + i2 + 1),
                        (x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2),
                    )
                })
        })
        .sorted_by_key(|(_, d)| *d)
        .map(|(elements, _)| elements)
        .collect()
}

pub fn part_one_(input: &str, amount: usize) -> Option<usize> {
    let points = parse_input(input);
    let pairs = sort_by_distance(&points);

    // Junction box i is part of group j
    let mut assignment: Vec<usize> = (0..points.len()).collect();

    // The set of all junction boxes in group j
    let mut groups: Vec<HashSet<usize>> = (0..points.len())
        .map(|i| std::iter::once(i).collect())
        .collect();

    for (i, j) in pairs.into_iter().take(amount) {
        // Move j into the group of i
        let current_assignment = assignment[j];
        let current_group = groups[current_assignment].clone();
        groups[current_assignment] = HashSet::new();

        for n in &current_group {
            assignment[*n] = assignment[i];
        }

        groups[assignment[i]].extend(current_group);
    }

    Some(
        groups
            .into_iter()
            .sorted_by_key(|g| -(g.len() as i64))
            .map(|g| g.len())
            .take(3)
            .product(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = parse_input(input);
    let pairs = sort_by_distance(&points);

    // Junction box i is part of group j
    let mut assignment: Vec<usize> = (0..points.len()).collect();

    // The set of all junction boxes in group j
    let mut groups: Vec<HashSet<usize>> = (0..points.len())
        .map(|i| std::iter::once(i).collect())
        .collect();

    for (i, j) in pairs.into_iter() {
        // Move j into the group of i
        let current_assignment = assignment[j];
        let current_group = groups[current_assignment].clone();
        groups[current_assignment] = HashSet::new();

        for n in &current_group {
            assignment[*n] = assignment[i];
        }

        groups[assignment[i]].extend(current_group);

        if groups[assignment[i]].len() == points.len() {
            return Some((points[i].0 * points[j].0) as u64);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
