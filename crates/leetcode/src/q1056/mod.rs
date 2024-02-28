#[cfg(test)]
mod test;

#[allow(dead_code)]
fn solution(n: i32) -> bool {
    let chars: Vec<char> = n.to_string().chars().collect();

    if chars
        .iter()
        .any(|x| *x == '2' || *x == '3' || *x == '4' || *x == '5' || *x == '7')
    {
        return false;
    }

    if chars.len() == 1 {
        return n == 6 || n == 9;
    }

    for i in 0..=(chars.len() / 2 - 1) {
        if chars[i] == '6' {
            if chars[chars.len() - i - 1] != '9' {
                return true;
            }
            continue;
        }
        if chars[i] == '9' {
            if chars[chars.len() - i - 1] != '6' {
                return true;
            }
            continue;
        }
        if chars[i] != chars[chars.len() - i - 1] {
            return true;
        }
    }

    false
}
