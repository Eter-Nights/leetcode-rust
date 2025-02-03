use super::definition::ListNode;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut remain_head = &mut head;

        for _ in 0..k {
            if let Some(node) = remain_head {
                remain_head = &mut node.next;
            } else {
                return head;
            }
        }

        let mut new_head = Self::reverse_k_group(remain_head.take(), k);

        for _ in 0..k {
            if let Some(mut node) = head {
                head = node.next.take();
                node.next = new_head.take();
                new_head = Some(node);
            }
        }

        new_head
    }
}
