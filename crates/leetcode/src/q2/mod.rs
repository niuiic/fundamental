#[cfg(test)]
mod test;

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[allow(dead_code)]
fn solution(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut result = ListNode::new(-1);
    let mut node = &mut result;

    let mut l1 = l1.unwrap();
    let mut l2 = l2.unwrap();

    let mut promotion = false;
    loop {
        let sum = if promotion {
            promotion = false;
            l1.val + l2.val + 1
        } else {
            l1.val + l2.val
        };

        if sum >= 10 {
            promotion = true;
            node.next = Some(Box::new(ListNode::new(sum - 10)));
        } else {
            node.next = Some(Box::new(ListNode::new(sum)));
        }
        node = node.next.as_mut().unwrap();

        if [&l1.next, &l2.next].iter().all(|x| x.is_none()) {
            break;
        }
        l1 = l1.next.unwrap_or(Box::new(ListNode::new(0)));
        l2 = l2.next.unwrap_or(Box::new(ListNode::new(0)));
    }

    if promotion {
        node.next = Some(Box::new(ListNode::new(1)));
    }

    result.next
}
