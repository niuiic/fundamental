use std::collections::HashSet;

#[allow(dead_code)]
fn solution(n: i32) -> Vec<String> {
    let n = n as usize;
    let mut memo: Vec<HashSet<String>> = vec![];

    for i in 0..=n {
        let mut cases = HashSet::<String>::new();

        if i == 1 {
            cases.insert("()".to_string());
            memo.push(cases);
            continue;
        }

        for j in 0..i {
            if j == 0 {
                memo[i - 1].iter().for_each(|x| {
                    cases.insert(format!("(){}", x));
                });
                continue;
            }

            if j == i - 1 {
                memo[i - 1].iter().for_each(|x| {
                    cases.insert(format!("({})", x));
                });
                continue;
            }

            memo[j].iter().for_each(|x| {
                memo[i - j - 1].iter().for_each(|y| {
                    cases.insert(format!("({}){}", x, y));
                });
            });
        }

        memo.push(cases);
    }

    memo[n].drain().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    fn do_test(input: i32, output: Vec<&str>) {
        let mut result = solution(input);
        let mut answer = output
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        result.sort();
        answer.sort();

        assert_eq!(result, answer);
    }

    #[test]
    fn case1() {
        do_test(3, vec!["((()))", "(()())", "(())()", "()(())", "()()()"]);
    }

    #[test]
    fn case2() {
        do_test(1, vec!["()"]);
    }

    #[test]
    fn case3() {
        do_test(
            4,
            vec![
                "(((())))", "((()()))", "((())())", "((()))()", "(()(()))", "(()()())", "(()())()",
                "(())(())", "(())()()", "()((()))", "()(()())", "()(())()", "()()(())", "()()()()",
            ],
        );
    }
}
