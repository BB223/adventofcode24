use aoc_utils::get_input;

#[tokio::main]
async fn main() {
    let input = get_input(6).await;
    let mut grid: Vec<Vec<char>> = vec![];
    input.lines().for_each(|l| grid.push(l.chars().collect()));
    let mut start: (usize, usize) = (500000, 500000);

    for (i, item) in grid.iter().enumerate() {
        for (j, itemj) in item.iter().enumerate() {
            match itemj {
                '^' | 'v' | '>' | '<' => start = (i, j),
                _ => continue,
            }
        }
    }

    run(start, &mut grid);

    let res = grid
        .iter()
        .map(|v| v.iter().filter(|c| **c == 'X').count())
        .sum::<usize>();

    println!("{res}");
}

fn run(position: (usize, usize), grid: &mut Vec<Vec<char>>) {
    let (mut nexti, mut nextj) = match grid[position.0][position.1] {
        '>' => (position.0, position.1 + 1),
        'v' => (position.0 + 1, position.1),
        '<' => (position.0, position.1 - 1),
        '^' => (position.0 - 1, position.1),
        _ => panic!("ups"),
    };
    if grid.get(nexti).is_none_or(|g| g.get(nextj).is_none()) {
        grid[position.0][position.1] = 'X';
        return;
    }

    if grid[nexti][nextj] == '#' {
        grid[position.0][position.1] = match grid[position.0][position.1] {
            '>' => 'v',
            'v' => '<',
            '<' => '^',
            '^' => '>',
            _ => panic!("ups"),
        };
        nexti = position.0;
        nextj = position.1;
    } else {
        grid[nexti][nextj] = grid[position.0][position.1];
        grid[position.0][position.1] = 'X';
    }

    run((nexti, nextj), grid);
}
