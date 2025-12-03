use fxhash::FxHashSet;
use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let sum = input.split(',')
        .map(|range| range.split('-').map(|num| num.parse::<u64>().unwrap()).next_tuple().unwrap())
        .map(|(start, end)| {
            let order = start.checked_ilog10().unwrap_or(0) + 1;
            let pow10 = 10_u64.pow(order / 2) * if order % 2 == 1 {10} else {1};

            let invalid_start = if order % 2 == 1 {
                pow10 / 10
            } else {
                start / pow10
            };
            let invalid_end = pow10 - 1;

            let mut sum = 0;
            for num in invalid_start..=invalid_end {
                let invalid = num * pow10 + num;

                if invalid < start {
                    continue;
                }

                if invalid > end {
                    break;
                }

                sum += invalid;
            }

            sum
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut set = FxHashSet::default();

    input.split(',')
        .map(|range| range.split('-').map(|num| num.parse::<u64>().unwrap()).next_tuple().unwrap())
        .for_each(|(start, end)| {
            let start_order = start.checked_ilog10().unwrap_or(0) + 1;
            let end_order = end.checked_ilog10().unwrap_or(0) + 1;

            let start_pow10 = 10_u64.pow(start_order / 2);
            let end_pow10 = 10_u64.pow(end_order / 2);

            let invalid_start = start / start_pow10;
            let mut invalid_end = end / end_pow10;

            if invalid_end < invalid_start {
                invalid_end *= 10;
            }

            'outer: for num in invalid_start..=invalid_end {

                let mut current = num;

                while  current > 0 {
                    let pow10 = 10_u64.pow(current.checked_ilog10().unwrap_or(0) + 1);

                    let mut candidate = current;
                    while candidate <= end {
                        candidate = candidate * pow10 + current;
                    }
                    candidate /= pow10;

                    if candidate > end {
                        break 'outer;
                    }

                    if candidate >= start && candidate > 10 && (candidate >= 100 || candidate != current) {
                        set.insert(candidate);
                    }

                    current /= 10;
                }
            }
        });

    Some(set.iter().sum())
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
