#[cfg(test)]
mod test;

#[allow(dead_code)]
fn solution(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }

    fn func(nums: &[i32]) -> i32 {
        let mut memo: Vec<(usize, i32)> = vec![];

        for i in 0..nums.len() {
            if i == 0 || i == 1 {
                memo.push((i, nums[i]));
                continue;
            }

            let mut max = 0;
            memo.iter().for_each(|x| {
                if x.0 >= i - 1 {
                    return;
                }
                if nums[i] + x.1 < max {
                    return;
                }
                max = nums[i] + x.1;
            });
            if max == 0 {
                continue;
            }
            memo.push((i, max));
        }

        memo.iter().map(|x| x.1).max().unwrap()
    }

    func(&nums[0..nums.len() - 1]).max(func(&nums[1..]))
}
