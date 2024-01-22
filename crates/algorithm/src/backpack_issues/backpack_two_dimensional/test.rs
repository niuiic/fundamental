use super::*;

struct Input {
    space: i32,
    weight: i32,
    items: Vec<(
        /* space */ i32,
        /* weight */ i32,
        /* value */ i32,
    )>,
}

fn test(input: Input, output: i32) {
    assert_eq!(
        solution(
            input.space,
            input.weight,
            input
                .items
                .iter()
                .map(|x| Item {
                    space: x.0,
                    weight: x.1,
                    value: x.2,
                })
                .collect(),
        ),
        output
    );
}

#[test]
fn case1() {
    test(
        Input {
            space: 5,
            weight: 6,
            items: vec![(1, 2, 3), (2, 4, 4), (3, 4, 5), (4, 5, 6)],
        },
        8,
    );
}
