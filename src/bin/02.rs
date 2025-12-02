advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .next()
            .unwrap()
            .split(',')
            .map(|x| {
                let mut x = x.split('-');
                let start: u64 = x.next().unwrap().parse().unwrap();
                let end: u64 = x.next().unwrap().parse().unwrap();

                let mut count = 0;

                for i in start..=end {
                    let x = i.to_string();
                    let (a, b) = x.split_at(x.len() / 2);

                    if a == b {
                        count += i;
                    }
                }

                count
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .next()
            .unwrap()
            .split(',')
            .map(|x| {
                let mut x = x.split('-');
                let start: u64 = x.next().unwrap().parse().unwrap();
                let end: u64 = x.next().unwrap().parse().unwrap();

                let mut count = 0;

                for i in start..=end {
                    let x = i.to_string();
                    let bytes = x.chars().collect::<Vec<_>>();

                    // The string can be split into 2, 3, 4, ..., n chunks
                    for number_of_chunks in 2..=x.len() {
                        // As long as all chunks have the same length
                        if x.len() % number_of_chunks != 0 {
                            continue;
                        }

                        let chunk_size = x.len() / number_of_chunks;
                        // println!("Checking '{}' chunks of size {}", number_of_chunks, chunk_size);

                        let result = bytes[..chunk_size].iter().enumerate().all(
                            |(i_offset, first_value)| {
                                // println!("Byte '{}' of chunk should be '{}'", i_offset, first_value);
                                (1..number_of_chunks).all(|chunk_i| {
                                    // println!("{} + {} * {}", i_offset, chunk_size, chunk_i);
                                    bytes[i_offset + chunk_size * chunk_i] == *first_value
                                })
                            },
                        );

                        if result {
                            count += i;
                            break;
                        }
                    }
                }

                count
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
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
