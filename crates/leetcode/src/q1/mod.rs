use std::collections::HashMap;

#[cfg(test)]
mod test;

#[allow(dead_code)]
fn solution(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }

    panic!()
}

#[allow(dead_code)]
fn solution2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::<i32, i32>::new();
    for i in 0..nums.len() {
        let left = target - nums[i];
        if map.contains_key(&left) {
            return vec![*map.get(&left).unwrap(), i as i32];
        }
        map.insert(nums[i], i as i32);
    }

    panic!()
}
