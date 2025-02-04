use super::definition::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(root) => {
                let left = Self::max_depth(root.borrow_mut().left.take());
                let right = Self::max_depth(root.borrow_mut().right.take());
                1 + max(left, right)
            }
            None => 0,
        }
    }
}
