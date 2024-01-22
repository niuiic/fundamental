#[cfg(test)]
mod test;

struct Item {
    space: i32,
    value: i32,
}

#[allow(dead_code)]
fn solution1(space: i32, items: Vec<Item>) -> i32 {
    fn dp(x: usize, y: i32, items: &Vec<Item>) -> i32 {
        if x == 0 {
            return 0;
        }

        let y_x = items[x - 1].space;
        let v_x = items[x - 1].value;
        if y < y_x {
            return dp(x - 1, y, items);
        }

        dp(x - 1, y, items).max(dp(x - 1, y - y_x, items) + v_x)
    }

    dp(items.len(), space, &items)
}

#[allow(dead_code)]
fn solution2(space: i32, items: Vec<Item>) -> i32 {
    let mut memo = Vec::<(/* space */ i32, /* value */ i32)>::new();

    for x in 0..items.len() {
        let mut memo_t = Vec::<(/* space */ i32, /* value */ i32)>::new();
        let y_x = items[x].space;
        let v_x = items[x].value;

        if x == 0 {
            memo_t.push((space - y_x, v_x));
            memo_t.push((space, 0));
        } else {
            memo.iter().for_each(|z| {
                if z.0 >= y_x {
                    memo_t.push((z.0 - y_x, z.1 + v_x));
                }
                memo_t.push(*z);
            })
        }

        memo = memo_t;
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
