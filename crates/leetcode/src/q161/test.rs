use super::*;

#[test]
fn case1() {
    assert_eq!(solution("ab".to_string(), "acb".to_string()), true);
}

#[test]
fn case2() {
    assert_eq!(solution("".to_string(), "".to_string()), false);
}

#[test]
fn case3() {
    assert_eq!(solution("a".to_string(), "".to_string()), true);
}

#[test]
fn case4() {
    assert_eq!(solution("a".to_string(), "ac".to_string()), true);
}
