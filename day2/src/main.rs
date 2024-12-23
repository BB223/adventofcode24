use aoc_utils::get_input;

#[tokio::main]
async fn main() {
    let input = get_input(2).await;
    let lines = input.lines();

    let safe = lines
        .into_iter()
        .filter(|report| {
            is_safe(
                report
                    .split_whitespace()
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect(),
            )
        })
        .count();

    println!("{safe}")
}

fn is_safe(report: Vec<u32>) -> bool {
    let all = report.windows(3).map(is_safe_win).all(|x| x);

    if all {
        return true;
    }

    for i in 0..report.len() {
        let mut check = report.clone();
        check.remove(i);
        if check.windows(3).map(is_safe_win).all(|x| x) {
            return true;
        }
    }

    false
}

fn is_safe_win(win: &[u32]) -> bool {
    let diff = win[0].abs_diff(win[1]);
    if diff == 0 || diff > 3 {
        return false;
    }

    let diff = win[1].abs_diff(win[2]);
    if diff == 0 || diff > 3 {
        return false;
    }

    if win[0] < win[1] && win[1] > win[2] {
        return false;
    }

    if win[0] > win[1] && win[1] < win[2] {
        return false;
    }

    true
}
