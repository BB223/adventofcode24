use aoc_utils::get_input;

#[tokio::main]
async fn main() {
    let input = get_input(4).await;
    let line_len = input.lines().next().unwrap().chars().count();
    let grid: Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();

    let top = |i: usize| i.wrapping_sub(line_len);
    let left = |i: usize| i.wrapping_sub(1);
    let right = |i: usize| i.wrapping_add(1);
    let bottom = |i: usize| i.wrapping_add(line_len);
    let top_left = |i: usize| top(left(i));
    let top_right = |i: usize| top(right(i));
    let bottom_left = |i: usize| bottom(left(i));
    let bottom_right = |i: usize| bottom(right(i));

    let res: usize = (0..grid.len())
        .filter(|i| grid[*i] == 'A')
        .filter(|i| {
            is_xmas(*i, 'M', top_left, &grid, line_len)
                && is_xmas(*i, 'S', bottom_right, &grid, line_len)
                && is_xmas(*i, 'M', top_right, &grid, line_len)
                && is_xmas(*i, 'S', bottom_left, &grid, line_len)
                || is_xmas(*i, 'M', top_left, &grid, line_len)
                    && is_xmas(*i, 'S', bottom_right, &grid, line_len)
                    && is_xmas(*i, 'S', top_right, &grid, line_len)
                    && is_xmas(*i, 'M', bottom_left, &grid, line_len)
                || is_xmas(*i, 'S', top_left, &grid, line_len)
                    && is_xmas(*i, 'M', bottom_right, &grid, line_len)
                    && is_xmas(*i, 'S', top_right, &grid, line_len)
                    && is_xmas(*i, 'M', bottom_left, &grid, line_len)
                || is_xmas(*i, 'S', top_left, &grid, line_len)
                    && is_xmas(*i, 'M', bottom_right, &grid, line_len)
                    && is_xmas(*i, 'M', top_right, &grid, line_len)
                    && is_xmas(*i, 'S', bottom_left, &grid, line_len)
        })
        .count();

    println!("{res}");
}

fn is_xmas(
    index: usize,
    cha: char,
    fun: impl Fn(usize) -> usize,
    grid: &[char],
    line_len: usize,
) -> bool {
    let next = fun(index);
    if grid.get(next).is_none_or(|c| *c != cha) {
        return false;
    }
    if index % line_len == line_len - 1 && next % line_len == 0 {
        return false;
    }
    if next % line_len == line_len - 1 && index % line_len == 0 {
        return false;
    }
    true
}
