use super::*;

struct Input {
    space: i32,
    item_groups: Vec<Vec<(/* space */ i32, /* value */ i32)>>,
}

fn test(input: Input, output: i32) {
    assert_eq!(
        solution(
            input.space,
            input
                .item_groups
                .iter()
                .map(|x| x
                    .iter()
                    .map(|y| Item {
                        space: y.0,
                        value: y.1
                    })
                    .collect())
                .collect()
        ),
        output
    );
}

#[test]
fn case1() {
    test(
        Input {
            space: 5,
            item_groups: vec![vec![(1, 2), (2, 4)], vec![(3, 4)], vec![(4, 5)]],
        },
        8,
    );
}
