use super::*;

fn test(nums: &mut Vec<i32>) {
    solution(nums);
    for i in 0..nums.len() - 1 {
        if i % 2 == 0 {
            assert_eq!(nums[i] <= nums[i + 1], true);
        } else {
            assert_eq!(nums[i] >= nums[i + 1], true);
        }
    }
}

#[test]
fn case1() {
    let mut nums = vec![3, 5, 1, 6, 2, 4];
    test(&mut nums);
}
