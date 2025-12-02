advent_of_code::solution!(1);

#[derive(Debug)]
enum Direction {
    Right,
    Left,
}

pub fn part_one(input: &str) -> Option<i64> {
    let (_, count) = input
        .lines()
        .map(|s| {
            let mut chars = s.chars();

            let direction = match chars.next().unwrap() {
                'R' => Direction::Right,
                'L' => Direction::Left,
                _ => panic!(),
            };

            (direction, chars.as_str().parse::<i64>().unwrap())
        })
        .fold((50, 0), |(current, count), (direction, amount)| {
            let next = match direction {
                Direction::Right => current + amount,
                Direction::Left => current - amount,
            } % 100;

            let mut count = count;

            if next == 0 {
                count += 1;
            }

            (next, count)
        });

    Some(count)
}

pub fn part_two(input: &str) -> Option<i64> {
    let (_, count) = input
        .lines()
        .map(|s| {
            let mut chars = s.chars();

            let direction = match chars.next().unwrap() {
                'R' => Direction::Right,
                'L' => Direction::Left,
                _ => panic!(),
            };

            (direction, chars.as_str().parse::<i64>().unwrap())
        })
        .fold((50, 0), |(current, count), (direction, amount)| {
            let zeros_passed = match direction {
                Direction::Right => {
                    // Count zeros passed: happens every 100 positions
                    (current + amount) / 100
                }
                Direction::Left => {
                    if current == 0 {
                        // Starting at 0, we hit 0 after 100, 200, ... steps
                        amount / 100
                    } else if amount >= current {
                        // We hit 0 after 'current' steps, then every 100 steps
                        (amount - current) / 100 + 1
                    } else {
                        // Don't reach 0
                        0
                    }
                }
            };

            let new_current = match direction {
                Direction::Right => (current + amount) % 100,
                Direction::Left => (current + 100 - amount % 100) % 100,
            };

            (new_current, count + zeros_passed)
        });

    Some(count)
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
        assert_eq!(result, Some(6));
    }
}
