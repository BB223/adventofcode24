use std::collections::VecDeque;

use aoc_utils::get_input;

#[tokio::main]
async fn main() {
    let input = get_input(9).await;

    let mut disk_layout = to_disk(input.trim());
    let mut non_frees = disk_layout
        .iter()
        .enumerate()
        .filter_map(|(i, val)| {
            if val.first().is_some_and(|c| c != ".") {
                Some(i)
            } else {
                None
            }
        })
        .rev()
        .collect();
    compress(&mut disk_layout, &mut non_frees);

    let nums: Vec<Option<u32>> = disk_layout
        .concat()
        .iter()
        .map(|c| c.parse().ok())
        .collect();
    let res = checksum(nums);

    println!("{res}")
}

fn to_disk(layout: &str) -> Vec<Vec<String>> {
    let mut count: i128 = -1;
    layout
        .chars()
        .enumerate()
        .map(|(i, val)| {
            if i % 2 == 0 {
                let many = val.to_digit(10).unwrap().try_into().unwrap();
                if many > 0 {
                    count = count.checked_add(1).unwrap();
                }
                vec![count.to_string(); many]
            } else {
                vec![".".to_string(); val.to_digit(10).unwrap().try_into().unwrap()]
            }
        })
        .filter(|d| !d.is_empty())
        .collect::<Vec<Vec<String>>>()
}

fn clean(disk: &mut Vec<Vec<String>>) {
    let mut i = 0;

    while i < disk.len() - 1 {
        if !disk[i].is_empty() && !disk[i + 1].is_empty() && disk[i + 1][0] == disk[i][0] {
            let next_vec = disk.remove(i + 1);
            disk[i].extend(next_vec);
        } else {
            i += 1;
        }
    }
}

fn compress(disk: &mut Vec<Vec<String>>, non_frees: &mut VecDeque<usize>) {
    let to_check = non_frees.pop_front();

    if to_check.is_none() {
        return;
    }

    let frees: VecDeque<usize> = disk
        .iter()
        .enumerate()
        .filter_map(|(i, val)| {
            if val.first().is_some_and(|c| c == ".") {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    for first_free in frees {
        let mut to_check = to_check.unwrap();
        if first_free > to_check {
            break;
        }

        let non_free_len = disk[to_check].len();
        let free_len = disk[first_free].len();

        if non_free_len > free_len {
            continue;
        }

        let diff = free_len - non_free_len;

        if diff > 0 {
            disk.insert(first_free + 1, vec![".".to_string(); diff]);
            for _ in 0..diff {
                disk[first_free].pop();
            }
            for i in 0..non_frees.len() {
                if non_frees[i] > first_free {
                    non_frees[i] += 1
                }
            }
            to_check += 1
        }

        disk.swap(first_free, to_check);
        clean(disk);
        break;
    }
    compress(disk, non_frees);
}

fn checksum(nums: Vec<Option<u32>>) -> u128 {
    nums.iter()
        .enumerate()
        .filter_map(|(i, &num)| Some((i as u128).checked_mul(num? as u128).unwrap()))
        .sum()
}
