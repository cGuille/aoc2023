use std::time::{Duration, Instant};

pub fn timed<R>(input: &str, f: fn(&str) -> R) -> (R, Duration) {
    let timer = Instant::now();
    let res = f(input);
    let elapsed = timer.elapsed();

    (res, elapsed)
}
