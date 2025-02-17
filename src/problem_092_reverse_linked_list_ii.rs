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
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut left = left as usize - 1;
        let mut right = right as usize - 1;

        let mut array = Self::link_list_to_array(head);

        while left < right {
            array.swap(left, right);
            left += 1;
            right -= 1;
        }

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
        let input_v = vec![1, 2, 3, 4, 5];

        let input_l = Solution::array_to_link_list(input_v);
        let output_l = Solution::reverse_between(input_l, 2, 4);
        let output_v = Solution::link_list_to_array(output_l);

        assert_eq!(output_v, vec![1, 4, 3, 2, 5]);
    }

    #[test]
    fn case_2() {
        let input_v = vec![3, 5];

        let input_l = Solution::array_to_link_list(input_v);
        let output_l = Solution::reverse_between(input_l, 1, 2);
        let output_v = Solution::link_list_to_array(output_l);

        assert_eq!(output_v, vec![5, 3]);
    }

    #[test]
    fn case_3() {
        let input_v = vec![3, 5];

        let input_l = Solution::array_to_link_list(input_v);
        let output_l = Solution::reverse_between(input_l, 1, 1);
        let output_v = Solution::link_list_to_array(output_l);

        assert_eq!(output_v, vec![3, 5]);
    }

    #[test]
    fn case_4() {
        let input_v = vec![3, 5];

        let input_l = Solution::array_to_link_list(input_v);
        let output_l = Solution::reverse_between(input_l, 2, 2);
        let output_v = Solution::link_list_to_array(output_l);

        assert_eq!(output_v, vec![3, 5]);
    }
}
