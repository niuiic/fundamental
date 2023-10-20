use super::*;

fn test(input: Vec<Vec<i32>>, output: i32) {
    assert_eq!(solution(input), output)
}

#[test]
fn case1() {
    test(vec![vec![1, 5, 3], vec![2, 9, 4]], 5);
}

#[test]
fn case2() {
    test(vec![vec![1, 3], vec![2, 4]], 5);
}
