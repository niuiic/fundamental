use super::*;

fn test(input: i32, output: Vec<&str>) {
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
    test(3, vec!["((()))", "(()())", "(())()", "()(())", "()()()"]);
}

#[test]
fn case2() {
    test(1, vec!["()"]);
}

#[test]
fn case3() {
    test(
        4,
        vec![
            "(((())))", "((()()))", "((())())", "((()))()", "(()(()))", "(()()())", "(()())()",
            "(())(())", "(())()()", "()((()))", "()(()())", "()(())()", "()()(())", "()()()()",
        ],
    );
}
