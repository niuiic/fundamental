use super::*;

#[test]
fn case1() {
    let mut arr = vec![
        't', 'h', 'e', ' ', 's', 'k', 'y', ' ', 'i', 's', ' ', 'b', 'l', 'u', 'e',
    ];
    solution(&mut arr);
    assert_eq!(
        arr,
        vec!['b', 'l', 'u', 'e', ' ', 'i', 's', ' ', 's', 'k', 'y', ' ', 't', 'h', 'e']
    )
}
