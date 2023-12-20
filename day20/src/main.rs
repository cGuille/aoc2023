use aocutil::timed;
use day20::parse;

fn main() {
    let input_path = std::env::args()
        .nth(1)
        .unwrap_or(String::from("/dev/stdin"));

    let input = std::fs::read_to_string(input_path).expect("Could not read input");

    let (res, timing) = timed(&input, part1);
    println!("Part 1: {res} ({timing:?})");

    // let (res, timing) = timed(&input, part2);
    // println!("Part 2: {res} ({timing:?})");
}

fn part1(input: &str) -> usize {
    let mut system = parse(input);

    for _ in 1..1000 {
        system.run().unwrap();
    }

    let stats = system.run().unwrap();

    stats.low_pulse_count * stats.high_pulse_count
}

fn part2(_input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &str = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";

    const SAMPLE_2: &str = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";

    #[test]
    fn part1_with_sample1() {
        assert_eq!(32000000, part1(SAMPLE_1));
    }

    #[test]
    fn part1_with_sample2() {
        assert_eq!(11687500, part1(SAMPLE_2));
    }

    #[test]
    #[ignore = "until part 2 is implemented"]
    fn part2_with_sample1() {
        assert_eq!(0, part2(SAMPLE_1));
    }

    #[test]
    #[ignore = "until part 2 is implemented"]
    fn part2_with_sample2() {
        assert_eq!(0, part2(SAMPLE_2));
    }
}
