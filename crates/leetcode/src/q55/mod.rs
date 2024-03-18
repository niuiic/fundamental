#[cfg(test)]
mod test;

#[allow(dead_code)]
fn solution(nums: Vec<i32>) -> bool {
    let mut farest = 0;
    let mut i = 0;

    while i <= farest && i < nums.len() - 1 {
        let farest_next = i + nums[i] as usize;
        if farest_next > farest {
            farest = farest_next
        }
        i = i + 1;
    }

    farest >= nums.len() - 1
}
