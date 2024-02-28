#[cfg(test)]
mod test;

#[allow(dead_code)]
fn solution(nums: &mut Vec<i32>) {
    nums.sort();

    if nums.len() < 3 {
        return;
    }

    for i in 1..nums.len() / 2 {
        swap(nums, i * 2 - 1, i * 2);
    }

    if nums.len() % 2 == 0 {
        swap(nums, nums.len() - 1, nums.len() - 3);
    } else {
        swap(nums, nums.len() - 1, nums.len() - 2)
    }
}

fn swap(nums: &mut Vec<i32>, i: usize, j: usize) {
    let temp = nums[i];
    nums[i] = nums[j];
    nums[j] = temp;
}
