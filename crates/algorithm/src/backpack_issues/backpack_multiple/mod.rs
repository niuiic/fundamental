#[cfg(test)]
mod test;

struct Item {
    space: i32,
    value: i32,
    count: i32,
}

#[allow(dead_code)]
fn solution(space: i32, items: Vec<Item>) -> i32 {
    let space = space as usize;
    let mut memo = vec![0; space + 1];

    for x in 0..items.len() {
        let y_x = items[x].space as usize;
        let v_x = items[x].value;
        let c_x = items[x].count as usize;

        for y in (y_x..=space).rev() {
            for i in 1..=c_x.min(y / y_x) {
                memo[y] = memo[y].max(memo[y - i * y_x] + i as i32 * v_x);
            }
        }
    }

    memo[space]
}
