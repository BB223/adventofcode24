use aoc_utils::get_input;

#[tokio::main]
async fn main() {
    let input = get_input(11).await;

    let mut stones: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    for _ in 0..25 {
        let mut tmp = vec![];
        for stone in stones {
            if stone == 0 {
                tmp.push(1);
            } else if stone.to_string().len() % 2 == 0 {
                let stone_str = stone.to_string();
                let (first, last) = stone_str.split_at(stone_str.len() / 2);

                tmp.push(first.parse().unwrap());
                tmp.push(last.parse().unwrap());
            } else {
                tmp.push(stone * 2024);
            }
        }
        stones = tmp;
    }

    let res = stones.len();

    println!("{res}");
}
