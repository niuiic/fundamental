#[cfg(test)]
mod test;

#[allow(dead_code)]
fn solution(s: String, shift: Vec<Vec<i32>>) -> String {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len() as i32;
    let mut start_pos = 0;

    shift.iter().for_each(|x| {
        let direction = x[0];
        let amount = x[1];

        if direction == 0 {
            start_pos = start_pos + amount;
            if start_pos >= len {
                start_pos = start_pos - len;
            }
        } else {
            start_pos = start_pos - amount;
            while start_pos < 0 {
                start_pos = start_pos + len;
            }
        }
    });

    let mut new_chars: Vec<String> = vec![];
    for i in start_pos..len {
        new_chars.push(chars[i as usize].to_string())
    }
    for i in 0..start_pos {
        new_chars.push(chars[i as usize].to_string())
    }
    new_chars.join("")
}
