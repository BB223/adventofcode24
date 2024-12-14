use std::collections::VecDeque;

use aoc_utils::get_input;

#[tokio::main]
async fn main() {
    let input = get_input(7).await;
    let mut res: Vec<u64> = vec![];
    for line in input.lines() {
        let mut splits: VecDeque<u64> = line
            .split(&[' ', ':'])
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        let target = splits.pop_front().unwrap();
        let current = splits.pop_front().unwrap();
        if is_true(splits, current, target) {
            res.push(target);
        }
    }
    let res: u64 = res.iter().sum();

    println!("{res}");
}

fn is_true(mut nums: VecDeque<u64>, current: u64, target: u64) -> bool {
    match nums.pop_front() {
        Some(apply) => {
            is_true(nums.clone(), current + apply, target)
                || is_true(nums.clone(), current * apply, target)
                || is_true(
                    nums.clone(),
                    current * 10u64.pow(apply.ilog10() + 1) + apply,
                    target,
                )
        }
        None => current == target,
    }
}
