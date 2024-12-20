use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
    thread,
};

use aoc_utils::get_input;

#[tokio::main]
async fn main() {
    let input = get_input(10).await;
    let map: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let shared_map = Arc::new(map);
    let mut handles = Vec::new();

    for (i, line) in shared_map.iter().enumerate() {
        for (j, &d) in line.iter().enumerate() {
            if d != 0 {
                continue;
            }

            let child_map = Arc::clone(&shared_map);
            let shared_nines = Arc::new(Mutex::new(HashSet::new()));
            let child_nines = Arc::clone(&shared_nines);

            let thread = thread::Builder::new().name(format!("{},{}", i, j));
            let handle = thread
                .spawn(move || {
                    finder(child_map, (i, j), Arc::clone(&child_nines));

                    ((i, j), child_nines.lock().unwrap().len())
                })
                .expect("I don't know");

            handles.push(handle);
        }
    }
    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.join().unwrap());
    }

    let res: usize = results.iter().map(|r| r.1).sum();

    println!("{res}");
}

fn finder(
    map: Arc<Vec<Vec<u32>>>,
    position: (usize, usize),
    nines: Arc<Mutex<HashSet<(usize, usize)>>>,
) {
    let (x, y) = position;
    let curr = map[x][y];
    if curr == 9 {
        let mut set = nines.lock().unwrap();
        set.insert(position);
    }
    let mut handles = Vec::new();

    if map[x].get(y - 1).is_some_and(|&d| d == curr + 1) {
        let child_map = Arc::clone(&map);
        let child_nines = Arc::clone(&nines);

        let handle = thread::spawn(move || finder(child_map, (x, y - 1), child_nines));

        handles.push(handle);
    }
    if map[x].get(y + 1).is_some_and(|&d| d == curr + 1) {
        let child_map = Arc::clone(&map);
        let child_nines = Arc::clone(&nines);

        let handle = thread::spawn(move || finder(child_map, (x, y + 1), child_nines));

        handles.push(handle);
    }
    if map.get(x + 1).is_some_and(|d| d[y] == curr + 1) {
        let child_map = Arc::clone(&map);
        let child_nines = Arc::clone(&nines);

        let handle = thread::spawn(move || finder(child_map, (x + 1, y), child_nines));

        handles.push(handle);
    }
    if map.get(x - 1).is_some_and(|d| d[y] == curr + 1) {
        let child_map = Arc::clone(&map);
        let child_nines = Arc::clone(&nines);

        let handle = thread::spawn(move || finder(child_map, (x - 1, y), child_nines));

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
