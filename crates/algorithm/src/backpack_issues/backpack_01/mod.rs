use std::{usize, vec};

#[cfg(test)]
mod test;

struct Item {
    space: i32,
    value: i32,
}

#[allow(dead_code)]
fn solution1(space: i32, items: Vec<Item>) -> i32 {
    fn dp(x: usize, y: i32, items: &Vec<Item>) -> i32 {
        let y_x = if x > 0 { items[x - 1].space } else { 0 };
        let v_x = if x > 0 { items[x - 1].value } else { 0 };

        if x == 0 || y < y_x {
            return 0;
        }

        dp(x - 1, y, items).max(dp(x - 1, y - y_x, items) + v_x)
    }

    dp(items.len() as usize, space, &items)
}

#[allow(dead_code)]
fn solution2(space: i32, items: Vec<Item>) -> i32 {
    let mut memo = Vec::<(/* space */ i32, /* value */ i32)>::new();

    for x in 0..items.len() {
        let mut temp = Vec::<(/* space */ i32, /* value */ i32)>::new();
        let y_x = items[x].space;
        let v_x = items[x].value;

        if x == 0 {
            temp.push((space - y_x, v_x));
            temp.push((space, 0));
        } else {
            memo.iter().for_each(|z| {
                if z.0 >= y_x {
                    temp.push((z.0 - y_x, z.1 + v_x));
                }
                temp.push(*z);
            })
        }

        memo = temp;
    }

    memo.iter().map(|z| z.1).max().unwrap()
}

#[allow(dead_code)]
fn solution3(space: i32, items: Vec<Item>) -> i32 {
    let space = space as usize;
    let mut memo = vec![0; space + 1];

    for x in 0..items.len() {
        let y_x = items[x].space as usize;
        let v_x = items[x].value;

        for y in (y_x..=space).rev() {
            memo[y] = memo[y].max(memo[y - y_x] + v_x);
        }
    }

    memo[space]
}
