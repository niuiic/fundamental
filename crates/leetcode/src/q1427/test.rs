use super::*;

fn test(s: &str, shift: Vec<Vec<i32>>, result: &str) {
    assert_eq!(solution(s.to_string(), shift), result.to_string());
}

#[test]
fn case1() {
    test("abc", vec![vec![0, 1], vec![1, 2]], "cab")
}
