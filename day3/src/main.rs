use aoc_utils::get_input;
use regex::Regex;

#[tokio::main]
async fn main() {
    let input = get_input(3).await;

    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let rs: i32 = regex
        .captures_iter(&input)
        .map(|m| {
            m.get(1).unwrap().as_str().parse::<i32>().unwrap()
                * m.get(2).unwrap().as_str().parse::<i32>().unwrap()
        })
        .sum();
    println!("{rs}");
}
