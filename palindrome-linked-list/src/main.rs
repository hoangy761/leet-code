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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
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

        let mut midle_reverse = Solution::reverse_list(slow.cloned());
        let mut cursor = head;
        while midle_reverse.is_some() {
            if midle_reverse.as_ref().unwrap().val != cursor.as_ref().unwrap().val {
                return false;
            }
            midle_reverse = midle_reverse.unwrap().next;
            cursor = cursor.unwrap().next;
        }
        true
    }

    fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev: Option<Box<ListNode>> = None;
        let mut cursor = head;

        while cursor.is_some() {
            let mut node = cursor.unwrap();
            let mut next: Option<Box<ListNode>> = node.next.take();

            node.next = prev;
            prev = Some(node);
            cursor = next.take()
        }
        prev
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
                    val: 2,
                    next: Some(Box::new(ListNode { val: 1, next: None })),
                })),
            })),
        })),
    }));
    let rs = Solution::is_palindrome(head);
    println!("Hello, world!:: {:?}", rs);
}
