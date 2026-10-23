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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut cursor = &mut head;

        while cursor.is_some() {
            let is_duplicate = {
                let node = cursor.as_ref().unwrap();
                node.next.as_ref().is_some_and(|next| node.val == next.val)
            };
            if is_duplicate {
                let next_next = cursor.as_mut().unwrap().next.as_mut().unwrap().next.take();
                cursor.as_mut().unwrap().next = next_next;
            } else {
                cursor = &mut cursor.as_mut().unwrap().next;
            }
        }

        head
    }
}

fn main() {
    let head = Some(Box::new(ListNode::new(3)));
    let rs = Solution::delete_duplicates(head);
    println!("Hello, world!:: {:?}", rs);
}
