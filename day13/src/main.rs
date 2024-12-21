use aoc_utils::get_input;
use regex::Regex;

struct Machine {
    button_a: (u32, u32),
    button_b: (u32, u32),
    prize: (u32, u32),
}
impl Machine {
    pub const fn new(button_a: (u32, u32), button_b: (u32, u32), prize: (u32, u32)) -> Self {
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
            let result: Vec<u32> = regex
                .find_iter(chunk[0])
                .map(|m| m.as_str().parse().unwrap())
                .collect();
            let button_a = (result[0], result[1]);
            let result: Vec<u32> = regex
                .find_iter(chunk[1])
                .map(|m| m.as_str().parse().unwrap())
                .collect();
            let button_b = (result[0], result[1]);
            let result: Vec<u32> = regex
                .find_iter(chunk[2])
                .map(|m| m.as_str().parse().unwrap())
                .collect();
            let prize = (result[0], result[1]);

            Machine::new(button_a, button_b, prize)
        })
        .collect();

    let res: u32 = machines
        .iter()
        .filter_map(|machine| {
            solve_linear_system(
                machine.button_a.0.into(),
                machine.button_b.0.into(),
                machine.prize.0.into(),
                machine.button_a.1.into(),
                machine.button_b.1.into(),
                machine.prize.1.into(),
            )
        })
        .map(|dinger| (dinger.0 * 3.0 + dinger.1) as u32)
        .sum();
    println!("{res}");
}

fn solve_linear_system(a1: f64, b1: f64, c1: f64, a2: f64, b2: f64, c2: f64) -> Option<(f64, f64)> {
    let determinant = a1 * b2 - a2 * b1;

    if determinant == 0.0 {
        // Keine eindeutige Lösung (entweder unendlich viele Lösungen oder keine Lösung)
        return None;
    }

    let a = (c1 * b2 - c2 * b1) / determinant;
    let b = (a1 * c2 - a2 * c1) / determinant;

    if (c1 * b2 - c2 * b1) % determinant != 0.0 || (a1 * c2 - a2 * c1) % determinant != 0.0 {
        return None;
    }

    Some((a, b))
}
