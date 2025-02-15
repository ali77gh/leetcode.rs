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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut array = Self::link_list_to_array(head);
        array.reverse();
        Self::array_to_link_list(array)
    }

    fn link_list_to_array(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut array = vec![];

        let mut current_node = head;

        while let Some(child) = current_node.take() {
            array.push(child.val);
            current_node = child.next;
        }

        array
    }

    fn array_to_link_list(array: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(array[0])));

        let mut current_node: Option<&mut Box<ListNode>> = head.as_mut();

        for i in array.into_iter().skip(1) {
            let new_node = Some(Box::new(ListNode::new(i)));
            current_node.as_mut().unwrap().next = new_node;
            current_node = current_node.unwrap().next.as_mut();
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut l1 = ListNode::new(1);
        let mut l2 = ListNode::new(2);
        let l3 = ListNode::new(3);

        l2.next = Some(Box::new(l3));
        l1.next = Some(Box::new(l2));

        let output = Solution::reverse_list(Some(Box::new(l1)));

        let node1 = output.unwrap();
        let node2 = node1.next.unwrap();
        let node3 = node2.next.unwrap();

        assert_eq!(node1.val, 3);
        assert_eq!(node2.val, 2);
        assert_eq!(node3.val, 1);
    }

    #[test]
    fn case_2() {
        let head = None;
        assert_eq!(Solution::reverse_list(head), None);
    }
}
