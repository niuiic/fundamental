#[cfg(test)]
mod test;

#[allow(dead_code)]
fn solution(nums: Vec<i32>) -> i32 {
    let mut memo = vec![-1; nums.len()];

    for n in 1..=nums.len() {
        if n == 1 {
            memo[n - 1] = 0;
            continue;
        }

        for i in 1..n {
            if memo[n - i - 1] != -1 && nums[n - i - 1] as usize >= i {
                if memo[n - 1] == -1 || memo[n - i - 1] + 1 < memo[n - 1] {
                    memo[n - 1] = memo[n - i - 1] + 1;
                }
            }
        }
    }

    memo[nums.len() - 1]
}
