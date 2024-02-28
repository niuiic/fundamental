use std::collections::HashSet;

#[cfg(test)]
mod test;

#[allow(dead_code)]
fn solution(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let mut left = 0;
    let mut right = 0;
    let mut result = 0;
    let mut set = HashSet::<char>::new();

    while right < chars.len() {
        if set.contains(&chars[right]) {
            while left < right {
                set.remove(&chars[left]);
                left = left + 1;
                if chars[left - 1] == chars[right] {
                    break;
                }
            }
        }

        set.insert(chars[right]);
        if right - left + 1 > result {
            result = right - left + 1;
        }
        right = right + 1;
    }

    result as i32
}
