use super::*;

#[test]
fn case1() {
    assert_eq!(solution(123), 321);
}

#[test]
fn case2() {
    assert_eq!(solution(-123), -321);
}

#[test]
fn case3() {
    assert_eq!(solution(1534236469), 0);
}

#[test]
fn case4() {
    assert_eq!(solution(-2147483648), 0);
}
