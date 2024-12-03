use aoc_utils::get_input;

#[tokio::main]
async fn main() {
    let input = get_input(1).await;
    let lines = input.lines();
    let mut llist: Vec<u32> = Vec::new();
    let mut rlist: Vec<u32> = Vec::new();

    for line in lines {
        let mut nums = line.split_whitespace();
        let left = nums.next().unwrap();
        let right = nums.next().unwrap();
        llist.push(left.parse::<u32>().unwrap());
        rlist.push(right.parse::<u32>().unwrap());
    }

    let sum = llist
        .iter()
        .map(|num| num * count_nums(&rlist, *num))
        .sum::<u32>();

    println!("{sum}")
}

fn count_nums(list: &Vec<u32>, num: u32) -> u32 {
    list.iter()
        .filter(|n| **n == num)
        .count()
        .try_into()
        .unwrap()
}
