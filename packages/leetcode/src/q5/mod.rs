#[allow(dead_code)]
fn solution(s: String) -> String {
    let mut memo = vec![vec![false; s.len()]; s.len()];
    let chars: Vec<char> = s.chars().collect();
    let mut pos = (0, 0);

    for span in 0..s.len() {
        for start_pos in 0..s.len() - span {
            let (start, end) = (start_pos, start_pos + span);

            if span == 0 {
                memo[start][end] = true;
                continue;
            }

            if chars[start] == chars[end] && (span == 1 || memo[start + 1][end - 1]) {
                memo[start][end] = true;

                if span > pos.1 - pos.0 {
                    pos = (start, end)
                }
            } else {
                memo[start][end] = false
            }
        }
    }

    (&s[pos.0..=pos.1]).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    fn do_test(input: &str, output: &str) {
        assert_eq!(solution(input.to_string()), output.to_string());
    }

    #[test]
    fn case1() {
        do_test("babad", "bab");
    }

    #[test]
    fn case2() {
        do_test("cbbd", "bb");
    }
}
