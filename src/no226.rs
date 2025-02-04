use super::definition::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack = Vec::new();
        stack.push(root.clone());

        while let Some(node) = stack.pop() {
            if let Some(node) = node {
                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();

                if left.is_some() {
                    stack.push(left.clone())
                }
                if right.is_some() {
                    stack.push(right.clone())
                }

                node.borrow_mut().left = right;
                node.borrow_mut().right = left;
            }
        }

        root
    }
}
