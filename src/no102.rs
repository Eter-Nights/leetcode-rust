use super::definition::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut stack = VecDeque::new();
        if let Some(node) = root {
            stack.push_back(node);
        }

        while !stack.is_empty() {
            let mut tmp = vec![];
            for _ in 0..stack.len() {
                if let Some(node) = stack.pop_front() {
                    tmp.push(node.borrow().val);
                    if let Some(left) = node.borrow().left.clone() {
                        stack.push_back(left);
                    }
                    if let Some(right) = node.borrow().right.clone() {
                        stack.push_back(right);
                    }
                }
            }
            result.push(tmp);
        }

        result
    }
}
