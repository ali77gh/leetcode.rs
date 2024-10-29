#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut p1 = l1;
        let mut p2 = l2;
        let mut sum = Box::new(ListNode::new(0));
        let mut p_sum = &mut sum;

        let mut carry = 0;
        loop {
            let mut new_value = match (&p1, &p2) {
                (None, None) => break,
                (Some(x1), None) => x1.val,
                (None, Some(x2)) => x2.val,
                (Some(x1), Some(x2)) => x1.val + x2.val,
            } + carry;
            (carry, new_value) = (new_value / 10, new_value % 10);

            // insert new node to linked list
            let new_node = Some(Box::new(ListNode::new(new_value)));
            p_sum.next = new_node;

            // move pointers
            p_sum = p_sum.next.as_mut().unwrap();
            // p1 = p1;
            p1 = p1.and_then(|node| node.next);
            p2 = p2.and_then(|node| node.next);
        }
        if carry > 0 {
            p_sum.next = Some(Box::new(ListNode::new(carry)));
        }

        sum.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let mut l1 = Option::Some(Box::new(ListNode::new(1)));
        let mut l2 = Option::Some(Box::new(ListNode::new(2)));
        let l3 = Option::Some(Box::new(ListNode::new(3)));
        l2.as_mut().unwrap().next = l3;
        l1.as_mut().unwrap().next = l2;
        let list_1 = l1;

        let mut l1 = Option::Some(Box::new(ListNode::new(4)));
        let mut l2 = Option::Some(Box::new(ListNode::new(5)));
        let l3 = Option::Some(Box::new(ListNode::new(6)));
        l2.as_mut().unwrap().next = l3;
        l1.as_mut().unwrap().next = l2;
        let list_2 = l1;

        let sum = Solution::add_two_numbers(list_1, list_2);

        let mut l1 = Option::Some(Box::new(ListNode::new(5)));
        let mut l2 = Option::Some(Box::new(ListNode::new(7)));
        let l3 = Option::Some(Box::new(ListNode::new(9)));
        l2.as_mut().unwrap().next = l3;
        l1.as_mut().unwrap().next = l2;
        let should_be_sum = l1;

        assert_eq!(sum, should_be_sum);
    }

    #[test]
    fn with_carry() {
        let mut l1 = Option::Some(Box::new(ListNode::new(7)));
        let mut l2 = Option::Some(Box::new(ListNode::new(8)));
        let l3 = Option::Some(Box::new(ListNode::new(9)));
        l2.as_mut().unwrap().next = l3;
        l1.as_mut().unwrap().next = l2;
        let list_1 = l1;

        let mut l1 = Option::Some(Box::new(ListNode::new(6)));
        let mut l2 = Option::Some(Box::new(ListNode::new(7)));
        let l3 = Option::Some(Box::new(ListNode::new(8)));
        l2.as_mut().unwrap().next = l3;
        l1.as_mut().unwrap().next = l2;
        let list_2 = l1;

        let sum = Solution::add_two_numbers(list_1, list_2);

        let mut l1 = Option::Some(Box::new(ListNode::new(3)));
        let mut l2 = Option::Some(Box::new(ListNode::new(6)));
        let mut l3 = Option::Some(Box::new(ListNode::new(8)));
        let l4 = Option::Some(Box::new(ListNode::new(1)));

        l3.as_mut().unwrap().next = l4;
        l2.as_mut().unwrap().next = l3;
        l1.as_mut().unwrap().next = l2;
        let should_be_sum = l1;

        assert_eq!(sum, should_be_sum);
    }
}
