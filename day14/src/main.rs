use aoc_utils::get_input;
use regex::Regex;

#[tokio::main]
async fn main() {
    let input = get_input(14).await;
    let time = 8149;
    const WIDTH: i32 = 101;
    const HEIGHT: i32 = 103;
    let regex = Regex::new(r"-?\d+").unwrap();
    let mut grid = [[0; WIDTH as usize]; HEIGHT as usize];
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
                ((position.0 + velocity.0 * time) % WIDTH + WIDTH) % WIDTH,
                ((position.1 + velocity.1 * time) % HEIGHT + HEIGHT) % HEIGHT,
            )
        })
        .filter(|&(x, y)| x != WIDTH / 2 && y != HEIGHT / 2)
        .for_each(|(x, y)| {
            grid[y as usize][x as usize] += 1;
        });

    for inner in grid {
        for num in inner {
            if num == 0 {
                print!(" ");
            } else {
                print!("#");
            }
        }
        println!();
    }
}
