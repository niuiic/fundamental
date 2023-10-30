use super::*;

fn test(input: i32, output: i32) {
    assert_eq!(solution(input), output)
}

#[test]
fn case1() {
    test(12, 3);
}

#[test]
fn case2() {
    test(13, 2)
}
