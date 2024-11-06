// Definition for singly-linked list.
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
    // 0ms beats 100%
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = list1;
        let mut l2 = list2;
        let mut new_list = ListNode::new(0); // I will pop this '0' at the end
        let mut new_list_ptr = &mut new_list;

        while let (Some(n1), Some(n2)) = (&l1, &l2) {
            if n1.val < n2.val {
                new_list_ptr.next = l1.take();
                new_list_ptr = new_list_ptr.next.as_mut().unwrap();
                l1 = new_list_ptr.next.take();
            } else {
                new_list_ptr.next = l2.take();
                new_list_ptr = new_list_ptr.next.as_mut().unwrap();
                l2 = new_list_ptr.next.take();
            }
        }

        new_list_ptr.next = l1.or(l2);
        new_list.next // pop first '0' from list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // list 1
        let mut n1 = Box::new(ListNode::new(1));
        let mut n2 = Box::new(ListNode::new(2));
        let n3 = Box::new(ListNode::new(4));
        n2.next = Some(n3);
        n1.next = Some(n2);
        let list1 = n1;

        // list 2
        let mut n1 = Box::new(ListNode::new(1));
        let mut n2 = Box::new(ListNode::new(3));
        let n3 = Box::new(ListNode::new(4));
        n2.next = Some(n3);
        n1.next = Some(n2);
        let list2 = n1;

        // result
        let mut n1 = Box::new(ListNode::new(1));
        let mut n2 = Box::new(ListNode::new(1));
        let mut n3 = Box::new(ListNode::new(2));
        let mut n4 = Box::new(ListNode::new(3));
        let mut n5 = Box::new(ListNode::new(4));
        let n6 = Box::new(ListNode::new(4));
        n5.next = Some(n6);
        n4.next = Some(n5);
        n3.next = Some(n4);
        n2.next = Some(n3);
        n1.next = Some(n2);
        let result = n1;

        // assert
        assert_eq!(
            Solution::merge_two_lists(Some(list1), Some(list2)),
            Some(result)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::merge_two_lists(None, None), None);
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::merge_two_lists(None, Some(Box::new(ListNode::new(0)))),
            Some(Box::new(ListNode::new(0)))
        );
    }
}
