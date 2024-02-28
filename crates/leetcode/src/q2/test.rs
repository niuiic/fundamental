use super::*;

fn test(l1: Vec<i32>, l2: Vec<i32>, result: Vec<i32>) {
    let res = solution(vec_to_list_node(l1), vec_to_list_node(l2));
    assert_eq!(list_node_is_eqal(res, vec_to_list_node(result)), true)
}

fn vec_to_list_node(list: Vec<i32>) -> Option<Box<ListNode>> {
    if list.len() == 0 {
        return None;
    }

    let mut result = ListNode::new(-1);
    let mut node = &mut result;

    for i in 0..list.len() {
        node.next = Some(Box::new(ListNode::new(list[i])));
        node = node.next.as_mut().unwrap();
    }

    result.next
}

fn list_node_is_eqal(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> bool {
    if [&l1, &l2].iter().all(|x| x.is_none()) {
        return true;
    }
    if [&l1, &l2].iter().any(|x| x.is_none()) {
        return false;
    }

    let mut l1 = *l1.unwrap();
    let mut l2 = *l2.unwrap();

    loop {
        if l1.val != l2.val {
            return false;
        }

        if [&l1.next, &l2.next].iter().all(|x| x.is_none()) {
            return true;
        }
        if [&l1.next, &l2.next].iter().any(|x| x.is_none()) {
            return false;
        }

        l1 = *l1.next.unwrap();
        l2 = *l2.next.unwrap();
    }
}

#[test]
fn case1() {
    test(vec![2, 4, 3], vec![5, 6, 4], vec![7, 0, 8])
}
