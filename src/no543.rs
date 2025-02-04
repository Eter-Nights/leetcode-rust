use super::definition::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;
        Self::depth(root, &mut result);
        result
    }

    fn depth(root: Option<Rc<RefCell<TreeNode>>>, result: &mut i32) -> i32 {
        match root {
            Some(root) => {
                let left = Self::depth(root.borrow_mut().left.take(), result);
                let right = Self::depth(root.borrow_mut().right.take(), result);

                *result = (*result).max(left + right);
                1 + left.max(right)
            }
            None => 0,
        }
    }
}
