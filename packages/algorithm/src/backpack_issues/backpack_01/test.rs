use super::*;

struct Input {
    space: i32,
    items: Vec<(/* space */ i32, /* value */ i32)>,
}

fn test(input: Input, output: i32) {
    [solution1, solution2, solution3]
        .iter()
        .enumerate()
        .for_each(|(i, x)| {
            assert_eq!(
                x(
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
                output,
                "on solution {}",
                i + 1
            )
        })
}

#[test]
fn case1() {
    test(
        Input {
            space: 5,
            items: vec![(1, 2), (2, 4), (3, 4), (4, 5)],
        },
        8,
    )
}
