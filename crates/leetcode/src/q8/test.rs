use super::*;

#[test]
fn case1() {
    assert_eq!(solution("   -42".to_string()), -42);
}

#[test]
fn case2() {
    assert_eq!(solution("words and 987".to_string()), 0);
}

#[test]
fn case3() {
    assert_eq!(solution("3.14159".to_string()), 3);
}

#[test]
fn case4() {
    assert_eq!(solution("+-12".to_string()), 0);
}

#[test]
fn case5() {
    assert_eq!(solution("".to_string()), 0);
}
