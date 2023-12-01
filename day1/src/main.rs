fn main() {
    let input_path = std::env::args()
        .nth(1)
        .unwrap_or(String::from("/dev/stdin"));

    let input = std::fs::read_to_string(input_path).expect("Could not read input");

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(extract_first_and_last_digits)
        .map(|(digit1, digit2)| u64::from_str_radix(&format!("{digit1}{digit2}"), 10).unwrap())
        .sum()
}

fn extract_first_and_last_digits(s: &str) -> (char, char) {
    let mut first = None;
    let mut last = None;

    for c in s.chars() {
        if c.is_digit(10) {
            if first.is_none() {
                first = Some(c);
            } else {
                last = Some(c);
            }
        }
    }

    if last.is_none() {
        last = first
    }

    (first.unwrap(), last.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

    #[test]
    fn part1_with_sample() {
        assert_eq!(142, part1(SAMPLE));
    }

    #[test]
    fn test_extract_first_and_last_digits() {
        assert_eq!(('1', '2'), extract_first_and_last_digits("12"));
        assert_eq!(('1', '2'), extract_first_and_last_digits("a1b2c"));
    }
}
