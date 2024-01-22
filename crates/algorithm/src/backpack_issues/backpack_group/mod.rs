#[cfg(test)]
mod test;

struct Item {
    space: i32,
    value: i32,
}

#[allow(dead_code)]
fn solution(space: i32, item_groups: Vec<Vec<Item>>) -> i32 {
    fn dp(x: usize, y: i32, item_groups: &Vec<Vec<Item>>) -> i32 {
        if x == 0 {
            return 0;
        }

        item_groups[x - 1]
            .iter()
            .map(|item| {
                let y_x_i = item.space;
                let v_x_i = item.value;
                if y < y_x_i {
                    dp(x - 1, y, item_groups)
                } else {
                    dp(x - 1, y, item_groups).max(dp(x - 1, y - y_x_i, item_groups) + v_x_i)
                }
            })
            .max()
            .unwrap()
    }

    dp(item_groups.len(), space, &item_groups)
}
