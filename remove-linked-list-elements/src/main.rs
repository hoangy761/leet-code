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
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut cursor = &mut head;

        while cursor.is_some() {
            let is_eq = cursor.as_ref().unwrap().val == val;
            if is_eq {
                *cursor = cursor.as_mut().unwrap().next.take();
            } else {
                cursor = &mut cursor.as_mut().unwrap().next;
            }
        }
        head
    }
}

fn main() {
    let head = Some(Box::new(ListNode::new(3)));
    let rs = Solution::remove_elements(head, 3);
    println!("Hello, world!:: {:?}", rs);
}
