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
        let mut head1 = head.clone();
        let cursor = &mut head1;
        let mut len = 0;
        while cursor.is_some() {
            *cursor = cursor.as_mut().unwrap().next.take();
            len += 1;
        }
        let midle = len / 2;
        let mut head2 = head.clone();
        let cursor2 = &mut head2;
        for _index in 0..midle {
            *cursor2 = cursor2.as_mut().unwrap().next.take();
        }
        head2
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
