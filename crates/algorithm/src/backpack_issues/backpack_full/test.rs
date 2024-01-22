use super::*;

struct Input {
    space: i32,
    items: Vec<(/* space */ i32, /* value */ i32)>,
}

fn test(input: Input, output: i32) {
    assert_eq!(
        solution(
            input.space,
            input
                .items
                .iter()
                .map(|y| Item {
                    space: y.0,
                    value: y.1
                })
                .collect()
        ),
        output
    )
}

#[test]
fn case1() {
    test(
        Input {
            space: 5,
            items: vec![(2, 4), (1, 2), (3, 4), (4, 5)],
        },
        10,
    );
}
