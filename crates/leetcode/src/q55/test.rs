use super::*;

#[test]
fn case1() {
    assert_eq!(solution(vec![2, 3, 1, 1, 4]), true);
}

#[test]
fn case2() {
    assert_eq!(solution(vec![0, 2, 3]), false);
}
