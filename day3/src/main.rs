use std::str::FromStr;

use chargrid::CharGrid;

fn main() {
    let input_path = std::env::args()
        .nth(1)
        .unwrap_or(String::from("/dev/stdin"));

    let input = std::fs::read_to_string(input_path).expect("Could not read input");

    println!("Part 1: {}", part1(&input));
    // println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let grid = CharGrid::from_str(input).unwrap();

    let mut part_numbers = Vec::new();
    let mut digits_acc = String::new();
    let mut acc_is_adj = false;

    for (index, cell) in grid.cells().enumerate() {
        if is_digit(cell) {
            digits_acc.push(*cell);

            if !acc_is_adj
                && grid
                    .adj_cells(index)
                    .iter()
                    .any(|val| *val != '.' && !is_digit(&val))
            {
                acc_is_adj = true;
            }
        }

        if !is_digit(cell) || grid.is_end_of_row(index) {
            if acc_is_adj {
                part_numbers.push(parse_i64(&digits_acc));
            }

            digits_acc.clear();
            acc_is_adj = false;
        }
    }

    part_numbers.iter().sum()
}

// fn part2(_input: &str) -> u64 {
//     todo!()
// }

fn is_digit(c: &char) -> bool {
    c.is_digit(10)
}

fn parse_i64(s: &str) -> i64 {
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

    // #[test]
    // #[ignore = "until part 2 is implemented"]
    // fn part2_with_sample() {
    //     assert_eq!(0, part2(SAMPLE));
    // }
}
