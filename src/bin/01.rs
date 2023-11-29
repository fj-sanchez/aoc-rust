use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    input
        .split_terminator("\n\n")
        .map(|group| {
            group
                .lines()
                .filter_map(|s| s.parse::<u32>().ok())
                .sum::<u32>()
        })
        .max()
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .split_terminator("\n\n")
            .map(|group: &str| {
                group
                    .lines()
                    .filter_map(|s: &str| s.parse::<u32>().ok())
                    .sum::<u32>()
            })
            .sorted()
            .rev()
            .take(3)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24000));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45000));
    }
}
