use aoc_utils::get_input;

#[tokio::main]
async fn main() {
    let input = get_input(1).await;
    let lines = input.lines();
    let mut llist: Vec<u32> = Vec::new();
    let mut rlist: Vec<u32> = Vec::new();
    let mut sum: u32 = 0;

    for line in lines {
        let mut nums = line.split_whitespace();
        let left = nums.next().unwrap();
        let right = nums.next().unwrap();
        llist.push(left.parse::<u32>().unwrap());
        rlist.push(right.parse::<u32>().unwrap());
    }

    llist.sort();
    rlist.sort();

    for i in 0..llist.len() {
        sum = sum + (llist[i].abs_diff(rlist[i]));
    }

    println!("{sum}")
}
