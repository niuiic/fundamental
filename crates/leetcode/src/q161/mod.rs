#[cfg(test)]
mod test;

#[allow(dead_code)]
fn solution(s: String, t: String) -> bool {
    let len_diff = s.len() as i32 - t.len() as i32;

    if len_diff.abs() > 1 {
        return false;
    }

    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();

    let longer_chars = if len_diff > 0 { &s_chars } else { &t_chars };
    let shorter_chars = if len_diff > 0 { &t_chars } else { &s_chars };

    let mut diff_count = 0;
    for i in 0..longer_chars.len() {
        let j = if len_diff == 0 { i } else { i - diff_count };
        if j >= shorter_chars.len() || longer_chars[i] != shorter_chars[j] {
            diff_count = diff_count + 1;

            if diff_count > 1 {
                return false;
            }
        }
    }

    diff_count == 1
}
