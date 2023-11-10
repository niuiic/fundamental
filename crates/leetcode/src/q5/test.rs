use super::*;

fn do_test(input: &str, output: &str) {
    assert_eq!(solution(input.to_string()), output.to_string());
}

#[test]
fn case1() {
    do_test("babad", "bab");
}

#[test]
fn case2() {
    do_test("cbbd", "bb");
}
