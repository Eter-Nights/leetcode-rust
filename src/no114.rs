use super::definition::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut stack = vec![];
        if let Some(node) = root {
            stack.push(node.clone());
        }
        let mut pre: Option<Rc<RefCell<TreeNode>>> = None;

        while let Some(node) = stack.pop() {
            if let Some(pre) = pre.clone() {
                pre.borrow_mut().right = Some(node.clone());
            }

            if let Some(right) = node.borrow_mut().right.take() {
                stack.push(right);
            }
            if let Some(left) = node.borrow_mut().left.take() {
                stack.push(left);
            }

            pre = Some(node);
        }
    }
}
