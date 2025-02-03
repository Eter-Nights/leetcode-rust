use super::definition::ListNode;

pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = ListNode::new(0);
        let mut cur = &mut result;
        while let (Some(n1), Some(n2)) = (&list1, &list2) {
            let lst = if n1.val < n2.val {
                &mut list1
            } else {
                &mut list2
            };
            cur.next = lst.take();
            cur = cur.next.as_mut()?;
            *lst = cur.next.take();
        }
        cur.next = list1.or(list2);

        result.next
    }
}
