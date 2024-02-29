#[cfg(test)]
mod test;

#[allow(dead_code)]
fn solution(x: i32) -> i32 {
    if x == i32::MIN {
        return 0;
    }
    if x.abs() < 10 {
        return x;
    }

    let mut is_negative = false;
    let mut chars: Vec<char> = if x < 0 {
        is_negative = true;
        x.abs().to_string().chars().collect()
    } else {
        x.to_string().chars().collect()
    };
    let mut left = 0;
    let mut right = chars.len() - 1;
    let mut result_chars: Vec<char> = vec![];

    for _ in 0..chars.len() / 2 {
        swap(&mut chars, left, right);
        left = left + 1;
        right = right - 1;
    }

    let mut has_first_not_zero = false;
    for char in chars {
        if char != '0' {
            has_first_not_zero = true;
        }

        if has_first_not_zero {
            result_chars.push(char)
        }
    }

    let result = result_chars
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<i64>()
        .unwrap();
    if result.abs() > i32::MAX as i64 {
        return 0;
    }
    if is_negative {
        return result as i32 * -1;
    }
    result as i32
}

fn swap(s: &mut Vec<char>, i: usize, j: usize) {
    let tmp = s[i];
    s[i] = s[j];
    s[j] = tmp;
}
