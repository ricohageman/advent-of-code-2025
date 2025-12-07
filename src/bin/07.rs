use std::collections::{HashMap, HashSet};

advent_of_code::solution!(7);

fn parse_input(input: &str) -> Vec<HashSet<usize>> {
    input
        .lines()
        .enumerate()
        .map(|(_, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| match c {
                    '.' => false,
                    'S' => true,
                    '^' => true,
                    _ => panic!("Unknown value '{c}'"),
                })
                .map(|(index, _)| index)
                .collect::<HashSet<usize>>()
        })
        .collect::<Vec<HashSet<usize>>>()
}

pub fn part_one(input: &str) -> Option<u64> {
    let splitters = parse_input(input);
    let mut beams = splitters[0].iter().copied().collect::<HashSet<_>>();
    assert_eq!(beams.len(), 1);

    let mut index = 2;
    let mut splits = 0;

    while index < splitters.len() {
        let splitters_at_level = &splitters[index];
        let mut new_beams = HashSet::new();

        for beam in &beams {
            if splitters_at_level.contains(beam) {
                new_beams.insert(*beam + 1);
                new_beams.insert(*beam - 1);

                splits += 1;
            } else {
                new_beams.insert(*beam);
            }
        }

        beams = new_beams;

        index += 2;
    }

    Some(splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let splitters = parse_input(input);

    let beams = splitters[0].iter().copied().collect::<Vec<_>>();
    assert_eq!(beams.len(), 1);

    // Starting at (height, position), there will be x paths to reach the end
    let mut cache: HashMap<(usize, usize), u64> = HashMap::new();

    Some(recursive_function(2, beams[0], &mut cache, &splitters))
}

pub fn recursive_function(
    height: usize,
    position: usize,
    cache: &mut HashMap<(usize, usize), u64>,
    splitters: &Vec<HashSet<usize>>,
) -> u64 {
    // Base case: We've passed all the splitters
    if height >= splitters.len() {
        return 1;
    }

    // If we've examined the path from here downwards, use the cached value.
    if let Some(remaining_paths) = cache.get(&(height, position)) {
        return *remaining_paths;
    }

    let splitters_at_level = &splitters[height];

    // If there is no splitter, continue downwards
    if !splitters_at_level.contains(&position) {
        return recursive_function(height + 2, position, cache, splitters);
    }

    // Otherwise, traverse both the left and the right path
    let left_paths = recursive_function(height + 2, position - 1, cache, splitters);
    let right_paths = recursive_function(height + 2, position + 1, cache, splitters);

    let number_of_paths = left_paths + right_paths;
    cache.insert((height, position), number_of_paths);

    number_of_paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
