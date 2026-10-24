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
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let arr = Solution::convert_linked_list_to_arr(head);
        let mut sum = 0;
        let len = arr.len();
        if len == 0 {
            return sum;
        }

        for (index, value) in arr.iter().enumerate() {
            sum = sum + value * 2_i32.pow((len - 1 - index) as u32)
        }

        sum
    }
    fn convert_linked_list_to_arr(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut cursor = head;
        let mut arr: Vec<i32> = Vec::new();
        while cursor.is_some() {
            let value = cursor.as_ref().unwrap().val;
            arr.push(value);
            cursor = cursor.unwrap().next;
        }
        arr
    }
    fn convert_arr_to_linked_list(arr: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        let mut cursor = &mut head;

        for value in arr {
            let node = Box::new(ListNode::new(value));
            *cursor = Some(node);

            cursor = &mut cursor.as_mut().unwrap().next;
        }
        head
    }
}

fn main() {
    let head =
        Solution::convert_arr_to_linked_list(vec![1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0]);
    let rs = Solution::get_decimal_value(head);
    println!("Hello, world!:: {:?}", rs);
}
