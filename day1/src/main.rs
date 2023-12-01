fn main() {
    let input_path = std::env::args()
        .nth(1)
        .unwrap_or(String::from("/dev/stdin"));

    let input = std::fs::read_to_string(input_path).expect("Could not read input");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
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

fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(extract_first_and_last_digits_v2)
        .map(|(digit1, digit2)| u64::from_str_radix(&format!("{digit1}{digit2}"), 10).unwrap())
        .sum()
}

fn extract_first_and_last_digits_v2(s: &str) -> (char, char) {
    let mut first = None;
    let mut last = None;

    for i in 0..s.len() {
        if let Some(n) = extract_digit(&s[i..]) {
            let c = Some(char::from_digit(n as u32, 10).unwrap());

            if first.is_none() {
                first = c;
            } else {
                last = c;
            }
        }
    }

    if last.is_none() {
        last = first
    }

    (first.unwrap(), last.unwrap())
}

const DIGITS: [&str; 20] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
    "five", "six", "seven", "eight", "nine",
];

fn extract_digit(s: &str) -> Option<usize> {
    for (index, digit_str) in DIGITS.iter().enumerate() {
        if s.starts_with(digit_str) {
            return Some(index % 10);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

    const SAMPLE_2: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

    #[test]
    fn part1_with_sample() {
        assert_eq!(142, part1(SAMPLE_1));
    }

    #[test]
    fn part2_with_sample() {
        assert_eq!(281, part2(SAMPLE_2));
    }

    #[test]
    fn test_extract_first_and_last_digits() {
        assert_eq!(('1', '2'), extract_first_and_last_digits("12"));
        assert_eq!(('1', '2'), extract_first_and_last_digits("a1b2c"));
    }

    #[test]
    fn test_extract_digit() {
        assert_eq!(None, extract_digit("abc"));
        assert_eq!(Some(0), extract_digit("0abc"));
        assert_eq!(Some(0), extract_digit("zeroabc"));
        assert_eq!(Some(1), extract_digit("one2three"));
    }

    #[test]
    fn test_extract_first_and_last_digits_v2() {
        assert_eq!(('1', '2'), extract_first_and_last_digits_v2("12"));
        assert_eq!(('1', '2'), extract_first_and_last_digits_v2("a1b2c"));
        assert_eq!(('3', '2'), extract_first_and_last_digits_v2("threea1b2c"));
        assert_eq!(('1', '7'), extract_first_and_last_digits_v2("a1b2cseven"));
    }
}
