#[cfg(test)]
mod test;

#[allow(dead_code)]
fn solution(s: &mut Vec<char>) {
    for i in 0..s.len() / 2 {
        swap(s, i, s.len() - i - 1);
    }

    let mut left_edge = 0;

    for i in 0..s.len() + 1 {
        if i == s.len() || s[i] == ' ' {
            for j in left_edge..(left_edge + (i - left_edge) / 2) {
                swap(s, j, i - j + left_edge - 1);
            }
            left_edge = i + 1;
        }
    }
}

fn swap(s: &mut Vec<char>, i: usize, j: usize) {
    let tmp = s[i];
    s[i] = s[j];
    s[j] = tmp;
}
