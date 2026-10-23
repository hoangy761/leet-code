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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = head.as_ref();
        let mut fast = head.as_ref();

        while fast.is_some() {
            if slow.is_none()
                || slow.as_ref().unwrap().next.is_none()
                || fast.as_ref().unwrap().next.is_none()
            {
                break;
            }

            slow = slow.as_ref().unwrap().next.as_ref();
            fast = fast.as_ref().unwrap().next.as_ref().unwrap().next.as_ref();
        }

        slow.cloned()
    }
}

fn main() {
    let head = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 5,
                        next: Some(Box::new(ListNode { val: 6, next: None })),
                    })),
                })),
            })),
        })),
    }));
    let rs = Solution::middle_node(head);
    println!("Hello, world!:: {:?}", rs);
}
