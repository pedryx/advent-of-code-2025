advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u16> {
    let (_, count) = input.lines()
        .map(|line| if line.starts_with('L') {-1} else {1} * line[1..].parse::<i16>().unwrap())
        .fold((50, 0), |(number, count), value| ((number + value) % 100, count + (number == 0) as u16));

    Some(count)
}

pub fn part_two(input: &str) -> Option<u16> {
    let (_, count) = input.lines()
        .map(|line| if line.starts_with('L') {-1} else {1} * line[1..].parse::<i16>().unwrap())
        .fold((50, 0), |(number, count), value| {
            let new_number = number + value;

            let mut new_count = count + (new_number / 100).unsigned_abs();
            if new_number <= 0 && number != 0 {
                new_count += 1;
            }

            (new_number.rem_euclid(100), new_count)
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
