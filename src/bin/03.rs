use rayon::prelude::*;

advent_of_code::solution!(3);

fn get_joltage(bank: &[u8], num_batteries: usize) -> u64 {
    let mut start_index = 0;
    let mut joltage = 0;

    for i in 0..num_batteries {
        let (index, battery) = bank.iter()
            .copied()
            .enumerate()
            .skip(start_index)
            .rev()
            .skip(num_batteries - i - 1)
            .max_by_key(|(_, battery)| *battery)
            .unwrap();

        start_index = index + 1;
        joltage = joltage * 10 + (battery - b'0') as u64;
    }

    joltage
}

pub fn part_one(input: &str) -> Option<u64> {
    let result = input.par_lines()
        .map(|bank| get_joltage(bank.as_bytes(), 2))
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = input.par_lines()
        .map(|bank| get_joltage(bank.as_bytes(), 12))
        .sum();

    Some(result)
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
