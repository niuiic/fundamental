use super::*;

#[test]
fn case1() {
    assert_eq!(solution(89), true);
}

#[test]
fn case2() {
    assert_eq!(solution(811), true);
}

#[test]
fn case3() {
    assert_eq!(solution(916), false);
}

#[test]
fn case4() {
    assert_eq!(solution(8101), true);
}

#[test]
fn case5() {
    assert_eq!(solution(910), true);
}

#[test]
fn case6() {
    assert_eq!(solution(0), false);
}

#[test]
fn case7() {
    assert_eq!(solution(8699), true);
}
