
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

    for line in lines {
        let nums: Vec<u32> = line.split(',').map(|s| s.parse().unwrap()).collect();

        let nums2 = correct_order(&rules, nums.clone());

        if nums != nums2 {
            middles.push(nums2[nums2.len() / 2]);
        }
    }

    let res: u32 = middles.iter().sum();

    println!("{res}");
}

fn correct_order(rules: &Vec<Vec<u32>>, mut nums: Vec<u32>) -> Vec<u32> {
    for rule in rules {
        let first = nums.iter().position(|&n| n == rule[0]);
        if first.is_none() {
            continue;
        }
        let second = nums.iter().position(|&n| n == rule[1]);
        if second.is_none() {
            continue;
        }

        if first > second {
            nums.swap(first.unwrap(), second.unwrap());
            let nums = correct_order(rules, nums);
            return nums;
        }
    }
    nums
}
