use super::*;

fn test(input: Vec<Vec<i32>>, output: i32) {
    assert_eq!(solution(input), output)
}

#[test]
fn case1() {
    test(vec![vec![17, 2, 17], vec![16, 16, 5], vec![14, 3, 19]], 10);
}

#[test]
fn case2() {
    test(vec![vec![7, 6, 2]], 2);
}
