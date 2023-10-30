use std::{usize, vec};

#[cfg(test)]
mod test;

struct Item {
    space: i32,
    value: i32,
}

#[allow(dead_code)]
fn solution(space: i32, items: Vec<Item>) -> i32 {
    let space = space as usize;
    let mut memo = vec![0; space + 1];

    for x in 0..items.len() {
        let y_x = items[x].space as usize;
        let v_x = items[x].value;

        for y in (y_x..=space).rev() {
            for i in 1..=(y / y_x) {
                memo[y] = memo[y].max(memo[y - i * y_x] + i as i32 * v_x);
            }
        }
    }

    memo[space]
}
