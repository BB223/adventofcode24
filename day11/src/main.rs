use std::collections::HashMap;

use aoc_utils::get_input;

#[tokio::main]
async fn main() {
    let input = get_input(11).await;

    let stones: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut cache = HashMap::with_capacity(100_000_000);
    let res: u64 = stones
        .iter()
        .map(|&stone| progress(stone, &mut cache, 0))
        .sum();

    println!("{res}");
}

fn progress(stone: u64, cache: &mut HashMap<String, u64>, blink: u8) -> u64 {
    if blink == 75 {
        return 1;
    }

    if let Some(&s) = cache.get(&(format!("{},{}", stone, blink))) {
        return s;
    }

    let ret = if stone == 0 {
        progress(1, cache, blink + 1)
    } else if num_digits_and_divisor(stone).0 % 2 == 0 {
        let (_, divisor) = num_digits_and_divisor(stone);
        let first = stone / divisor;
        let last = stone % divisor;
        progress(first, cache, blink + 1) + progress(last, cache, blink + 1)
    } else {
        progress(stone * 2024, cache, blink + 1)
    };

    cache.insert(format!("{},{}", stone, blink), ret);

    ret
}

fn num_digits_and_divisor(n: u64) -> (usize, u64) {
    const POWERS_OF_10: [u64; 20] = [
        1,
        10,
        100,
        1_000,
        10_000,
        100_000,
        1_000_000,
        10_000_000,
        100_000_000,
        1_000_000_000,
        10_000_000_000,
        100_000_000_000,
        1_000_000_000_000,
        10_000_000_000_000,
        100_000_000_000_000,
        1_000_000_000_000_000,
        10_000_000_000_000_000,
        100_000_000_000_000_000,
        1_000_000_000_000_000_000,
        10_000_000_000_000_000_000,
    ];

    let mut num = n;
    let mut digits = 0;

    while num >= 10 {
        num /= 10;
        digits += 1;
    }
    digits += 1;

    let divisor = POWERS_OF_10[digits / 2];
    (digits, divisor)
}
