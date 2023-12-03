use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use chargrid::CharGrid;

fn main() {
    let input_path = std::env::args()
        .nth(1)
        .unwrap_or(String::from("/dev/stdin"));

    let input = std::fs::read_to_string(input_path).expect("Could not read input");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u64 {
    let grid = CharGrid::from_str(input).unwrap();

    let mut part_numbers = Vec::new();
    let mut digits_acc = String::new();
    let mut acc_is_adj = false;

    for (index, cell) in grid.cells().enumerate() {
        if is_digit(cell) {
            digits_acc.push(*cell);

            if !acc_is_adj
                && grid
                    .adj_vals(index)
                    .into_iter()
                    .any(|val| val != '.' && !is_digit(&val))
            {
                acc_is_adj = true;
            }
        }

        if !is_digit(cell) || grid.is_end_of_row(index) {
            if acc_is_adj {
                part_numbers.push(parse_u64(&digits_acc));
            }

            digits_acc.clear();
            acc_is_adj = false;
        }
    }

    part_numbers.iter().sum()
}

fn part2(input: &str) -> u64 {
    let grid = CharGrid::from_str(input).unwrap();

    let mut gears_parts: HashMap<usize, Vec<u64>> = HashMap::new();
    let mut digits_acc = String::new();
    let mut adj_gears: HashSet<usize> = HashSet::new();

    for (index, cell) in grid.cells().enumerate() {
        if is_digit(cell) {
            digits_acc.push(*cell);

            adj_gears.extend(grid.adj_cells(index).into_iter().filter_map(|(pos, val)| {
                if val == '*' {
                    Some(pos)
                } else {
                    None
                }
            }));
        }

        if !is_digit(cell) || grid.is_end_of_row(index) {
            if digits_acc.len() > 0 {
                let part_number = parse_u64(&digits_acc);

                for gear_pos in adj_gears.iter() {
                    gears_parts
                        .entry(*gear_pos)
                        .or_insert(Vec::new())
                        .push(part_number);
                }
            }

            digits_acc.clear();
            adj_gears.clear();
        }
    }

    gears_parts
        .values()
        .filter(|part_numbers| part_numbers.len() >= 2)
        .map(|part_numbers| part_numbers.into_iter().fold(1, |acc, val| acc * val))
        .sum()
}

fn is_digit(c: &char) -> bool {
    c.is_digit(10)
}

fn parse_u64(s: &str) -> u64 {
    s.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    const SAMPLE_2: &str = "\
467..114..
...*......
..35...633
1.....#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn part1_with_sample() {
        assert_eq!(4361, part1(SAMPLE));
    }

    #[test]
    fn part1_with_sample2() {
        assert_eq!(4361, part1(SAMPLE_2));
    }

    #[test]
    fn part2_with_sample() {
        assert_eq!(467835, part2(SAMPLE));
    }

    #[test]
    fn part2_with_sample2() {
        assert_eq!(467835, part2(SAMPLE_2));
    }
}
