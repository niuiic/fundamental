use super::*;

#[test]
fn case1() {
    assert_eq!(solution(vec![vec![1, 2, 3], vec![4, 5], vec![1, 2, 3]]), 4);
    assert_eq!(solution2(vec![vec![1, 2, 3], vec![4, 5], vec![1, 2, 3]]), 4);
}

#[test]
fn case2() {
    assert_eq!(solution(vec![vec![1, 5], vec![3, 4]]), 3);
    assert_eq!(solution2(vec![vec![1, 5], vec![3, 4]]), 3);
}
