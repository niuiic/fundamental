use super::*;

struct Input {
    space: i32,
    items: Vec<(
        /* space */ i32,
        /* value */ i32,
        /* count */ i32,
    )>,
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
                    value: y.1,
                    count: y.2
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
            items: vec![(1, 2, 1), (2, 4, 1), (3, 4, i32::MAX), (4, 5, 2)],
        },
        8,
    );
}
