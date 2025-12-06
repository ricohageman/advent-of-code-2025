advent_of_code::solution!(3);

fn execute(n: usize, bank: &[u64]) -> u64 {
    let mut number = 0;
    let mut next_index = 0;

    for remaining_choices_afterwards in (0..n).rev() {
        let mut next_highest = 0;
        let mut next_highest_index = 0;

        for index in next_index..bank.len() - remaining_choices_afterwards {
            if bank[index] > next_highest {
                next_highest = bank[index];
                next_highest_index = index;
            }

            if next_highest == 9 {
                break;
            }
        }

        number = number * 10 + next_highest;
        next_index = next_highest_index + 1;
    }
    number
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|bank| {
                let bank = bank
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u64)
                    .collect::<Vec<_>>();

                execute(2, &bank)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|bank| {
                let bank = bank
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u64)
                    .collect::<Vec<_>>();

                execute(12, &bank)
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
