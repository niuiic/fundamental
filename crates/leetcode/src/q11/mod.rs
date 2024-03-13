#[cfg(test)]
mod test;

#[allow(dead_code)]
fn solution(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut max_size = 0;

    while left < right {
        max_size =
            area_size((left as i32, height[left]), (right as i32, height[right])).max(max_size);
        if height[left] <= height[right] {
            left = left + 1;
        } else {
            right = right - 1;
        }
    }

    max_size
}

fn area_size(left: (i32, i32), right: (i32, i32)) -> i32 {
    (right.0 - left.0) * left.1.min(right.1)
}
