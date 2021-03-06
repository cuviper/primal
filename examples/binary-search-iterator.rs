use std::env;
use std::time::Instant;

fn main() {
    let mut args = env::args();
    let zeros = args
        .nth(1).and_then(|s| s.parse::<f64>().ok().map(|x| x as u32))
        .unwrap_or(10);

    let start = Instant::now();
    let p = primal::Primes::all().find(|p| (p / 2).trailing_zeros() >= zeros).unwrap();
    let time = start.elapsed();

    println!("{} is the first prime with {} zeros, in {:?}",
             p, zeros, time);
}
