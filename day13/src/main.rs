use aoc_utils::get_input;
use regex::Regex;

struct Machine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}
impl Machine {
    pub const fn new(button_a: (i64, i64), button_b: (i64, i64), prize: (i64, i64)) -> Self {
        Machine {
            button_a,
            button_b,
            prize,
        }
    }
}
#[tokio::main]
async fn main() {
    let input = get_input(13).await;

    let lines: Vec<&str> = input.lines().filter(|&line| !line.is_empty()).collect();
    let regex = Regex::new(r"\d+").unwrap();
    let machines: Vec<Machine> = lines
        .chunks(3)
        .map(|chunk| {
            let result: Vec<i64> = regex
                .find_iter(chunk[0])
                .map(|m| m.as_str().parse().unwrap())
                .collect();
            let button_a = (result[0], result[1]);
            let result: Vec<i64> = regex
                .find_iter(chunk[1])
                .map(|m| m.as_str().parse().unwrap())
                .collect();
            let button_b = (result[0], result[1]);
            let result: Vec<i64> = regex
                .find_iter(chunk[2])
                .map(|m| m.as_str().parse().unwrap())
                .collect();
            let prize = (
                result[0] + 10_000_000_000_000,
                result[1] + 10_000_000_000_000,
            );

            Machine::new(button_a, button_b, prize)
        })
        .collect();

    let res: i64 = machines
        .iter()
        .filter_map(|machine| {
            solve_linear_system(
                machine.button_a.0,
                machine.button_b.0,
                machine.prize.0,
                machine.button_a.1,
                machine.button_b.1,
                machine.prize.1,
            )
        })
        .map(|dinger| dinger.0 * 3 + dinger.1)
        .sum();
    println!("{res}");
}

fn solve_linear_system(a1: i64, b1: i64, c1: i64, a2: i64, b2: i64, c2: i64) -> Option<(i64, i64)> {
    let determinant = a1 * b2 - a2 * b1;

    if determinant == 0 {
        // Keine eindeutige Lösung (entweder unendlich viele Lösungen oder keine Lösung)
        return None;
    }

    if (c1 * b2 - c2 * b1) % determinant != 0 || (a1 * c2 - a2 * c1) % determinant != 0 {
        return None;
    }

    let a = (c1 * b2 - c2 * b1) / determinant;
    let b = (a1 * c2 - a2 * c1) / determinant;

    Some((a, b))
}
