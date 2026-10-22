// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut merged_list = list1;

        if merged_list.is_none() {
            return list2;
        }

        let mut insertion_point = &mut merged_list;
        let mut list2_current = list2;

        while list2_current.is_some() {
            while insertion_point.is_some()
                && insertion_point.as_ref().unwrap().val <= list2_current.as_ref().unwrap().val
            {
                insertion_point = &mut insertion_point.as_mut().unwrap().next;
            }

            let mut node_to_insert = list2_current.take().unwrap();
            list2_current = node_to_insert.next.take();

            node_to_insert.next = insertion_point.take();
            *insertion_point = Some(node_to_insert);
        }

        merged_list
    }
}

fn main() {
    let list1 = Some(Box::new(ListNode { val: 1, next: None }));

    let list2 = Some(Box::new(ListNode { val: 2, next: None }));

    let result = Solution::merge_two_lists(list1, list2);

    println!("{:?}", result.unwrap());
}
