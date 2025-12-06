advent_of_code::solution!(6);

#[derive(Debug, Copy, Clone)]
enum Operation {
    Multiplication,
    Addition,
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines().collect::<Vec<_>>();

    let values = lines[0..lines.len() - 1]
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|e| e.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let operations = lines[lines.len() - 1]
        .split_whitespace()
        .map(|e| match e {
            "*" => Operation::Multiplication,
            "+" => Operation::Addition,
            _ => panic!("Unknown value {}", e),
        })
        .collect::<Vec<_>>();

    let mut running_sum = 0;

    for (index, operation) in operations.iter().enumerate() {
        let mut acc: u64 = match operation {
            Operation::Multiplication => 1,
            Operation::Addition => 0,
        };

        for values in &values {
            match operation {
                Operation::Multiplication => {
                    acc = acc * values[index];
                }
                Operation::Addition => {
                    acc = acc + values[index];
                }
            }
        }

        running_sum += acc;
    }

    Some(running_sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines().collect::<Vec<_>>();

    let largest_line = lines.iter().map(|line| line.len()).max().unwrap();

    let values = lines[0..lines.len() - 1]
        .iter()
        .map(|line| {
            line.chars()
                .map(|e| e.to_digit(10))
                .chain(std::iter::repeat(None))
                .take(largest_line)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let operations = lines[lines.len() - 1]
        .chars()
        .map(|e| match e {
            '*' => Some(Operation::Multiplication),
            '+' => Some(Operation::Addition),
            ' ' => None,
            _ => panic!("Unknown value {}", e),
        })
        .chain(std::iter::repeat(None))
        .take(largest_line)
        .collect::<Vec<_>>();

    let mut running_sum = 0;
    let mut numbers = vec![];

    for index in (0..values[0].len()).rev() {
        let mut acc = 0;

        for values in &values {
            let Some(value) = values[index] else {
                continue;
            };

            if acc != 0 {
                acc *= 10;
            }

            acc += value as u64;
        }

        if acc == 0 {
            continue;
        }

        numbers.push(acc);

        let Some(operation) = &operations[index] else {
            continue;
        };

        match operation {
            Operation::Multiplication => {
                running_sum += numbers.iter().fold(1, |acc, x| acc * x);
            }
            Operation::Addition => {
                running_sum += numbers.iter().fold(0, |acc, x| acc + x);
            }
        }

        numbers.clear();
    }

    Some(running_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
