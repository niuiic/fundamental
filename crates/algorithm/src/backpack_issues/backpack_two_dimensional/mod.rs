#[cfg(test)]
mod test;

struct Item {
    space: i32,
    weight: i32,
    value: i32,
}

#[allow(dead_code)]
fn solution(space: i32, weight: i32, items: Vec<Item>) -> i32 {
    fn dp(x: usize, y: i32, z: i32, items: &Vec<Item>) -> i32 {
        if x == 0 {
            return 0;
        }

        let y_x = items[x - 1].space;
        let z_x = items[x - 1].weight;
        let v_x = items[x - 1].value;
        if y < y_x || z < z_x {
            return dp(x - 1, y, z, items);
        }

        dp(x - 1, y, z, items).max(dp(x - 1, y - y_x, z - z_x, items) + v_x)
    }

    dp(items.len(), space, weight, &items)
}
