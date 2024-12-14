use aoc_utils::get_input;

#[tokio::main]
async fn main() {
    let input = get_input(5).await;
    let mut lines = input.lines();
    let mut rules: Vec<Vec<u32>> = vec![];
    let mut middles: Vec<u32> = vec![];

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let nums: Vec<u32> = line.split('|').map(|s| s.parse().unwrap()).collect();
        rules.push(nums);
    }

    'outer: for line in lines {
        let nums = line.split(',').map(|s| s.parse::<u32>().unwrap());

        for rule in &rules {
            let first = nums.clone().position(|n| n == rule[0]);
            if first.is_none() {
                continue;
            }
            let second = nums.clone().position(|n| n == rule[1]);
            if second.is_none() {
                continue;
            }

            if first > second {
                continue 'outer;
            }
        }

        let nums: Vec<u32> = nums.collect();
        middles.push(nums[nums.len() / 2]);
    }

    let res: u32 = middles.iter().sum();

    println!("{res}");
}
