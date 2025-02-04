use std::cmp::Reverse;
use std::collections::BinaryHeap;
use super::definition::ListNode;


struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();
        let mut result = None;
        let mut cur = &mut result;

        for list in lists.into_iter() {
            if let Some(node) = list {
                heap.push(Reverse(node));
            }
        }

        while let Some(mut node) = heap.pop() {
            if let Some(next) = node.0.next.take() {
                heap.push(Reverse(next));
            }
            cur = &mut cur.insert(node.0).next;
        }

        result
    }
}