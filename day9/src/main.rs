use aoc_utils::get_input;

#[tokio::main]
async fn main() {
    let input = get_input(9).await;

    let mut disk_layout = to_disk(input.trim());
    compress(&mut disk_layout);

    let nums: Vec<u32> = disk_layout
        .iter()
        .filter(|&c| c != ".")
        .map(|c| c.parse().unwrap())
        .collect();
    let res = checksum(nums);

    println!("{res}")
}

fn to_disk(layout: &str) -> Vec<String> {
    let mut count: i128 = -1;
    layout
        .chars()
        .enumerate()
        .map(|(i, val)| {
            if i % 2 == 0 {
                count = count.checked_add(1).unwrap();
                vec![count.to_string(); val.to_digit(10).unwrap().try_into().unwrap()]
            } else {
                vec![".".to_string(); val.to_digit(10).unwrap().try_into().unwrap()]
            }
        })
        .collect::<Vec<Vec<String>>>()
        .concat()
}

fn compress(disk: &mut Vec<String>) {
    let first_free = disk.iter().position(|c| c == ".");
    let last_non_free = disk.iter().rposition(|c| c != ".");

    if first_free.is_none() || last_non_free.is_none() {
        return;
    }

    if first_free > last_non_free {
        return;
    }

    disk.swap(first_free.unwrap(), last_non_free.unwrap());
    compress(disk);
}

fn checksum(nums: Vec<u32>) -> u128 {
    nums.iter()
        .enumerate()
        .map(|(i, &num)| (i as u128).checked_mul(num as u128).unwrap())
        .sum()
}
