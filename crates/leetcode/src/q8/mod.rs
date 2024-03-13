#[cfg(test)]
mod test;

#[allow(dead_code)]
fn solution(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let mut left = 0;
    let mut left_set = false;
    let mut right = 0;

    for i in 0..chars.len() {
        if !left_set {
            if is_num(chars[i]) || chars[i] == '+' || chars[i] == '-' {
                left = i;
                left_set = true;
                right = left;
            } else if chars[i] != ' ' {
                return 0;
            }
        } else {
            if is_num(chars[i]) {
                right = i;
            } else {
                break;
            }
        }
    }

    if !left_set || !is_num(chars[right]) {
        return 0;
    }

    let str = chars[left..=right]
        .into_iter()
        .map(|x| x.to_string())
        .reduce(|a, b| format!("{}{}", a, b))
        .unwrap();
    match i32::from_str_radix(&str, 10) {
        Ok(num) => num,
        Err(_) => {
            if chars[left] == '-' {
                i32::MIN
            } else {
                i32::MAX
            }
        }
    }
}

fn is_num(c: char) -> bool {
    let num_chars = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    num_chars.iter().any(|x| *x == c)
}
