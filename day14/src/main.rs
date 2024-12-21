use aoc_utils::get_input;
use regex::Regex;

#[tokio::main]
async fn main() {
    let input = get_input(14).await;
    let time = 100;
    let width = 101;
    let height = 103;
    let regex = Regex::new(r"-?\d+").unwrap();
    let mut results = [0; 4];
    input
        .lines()
        .map(|line| {
            let mut cords = regex
                .find_iter(line)
                .map(|m| m.as_str().parse::<i32>().unwrap());
            let position = (cords.next().unwrap(), cords.next().unwrap());
            let velocity = (cords.next().unwrap(), cords.next().unwrap());
            (position, velocity)
        })
        .map(|(position, velocity)| {
            (
                ((position.0 + velocity.0 * time) % width + width) % width,
                ((position.1 + velocity.1 * time) % height + height) % height,
            )
        })
        .filter(|&(x, y)| x != width / 2 && y != height / 2)
        .for_each(|(x, y)| {
            if x < width / 2 {
                if y < height / 2 {
                    results[0] += 1;
                } else {
                    results[1] += 1;
                }
            } else if y < height / 2 {
                results[2] += 1;
            } else {
                results[3] += 1;
            }
        });
    let res: u32 = results.iter().product();
    println!("{res}");
}
