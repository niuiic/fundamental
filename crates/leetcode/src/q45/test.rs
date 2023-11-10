use super::*;

fn test(input: Vec<i32>, output: i32) {
    assert_eq!(solution(input), output)
}

#[test]
fn case1() {
    test(vec![2, 3, 1, 1, 4], 2)
}

#[test]
fn case2() {
    test(vec![2, 3, 0, 1, 4], 2)
}

#[test]
fn case3() {
    test(vec![1], 0)
}
