use aoc_utils::get_input;

#[tokio::main]
async fn main() {
    let input = get_input(5).await;
    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

    }
}
