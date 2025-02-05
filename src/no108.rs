use super::definition::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(&nums)
    }

    fn helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }

        let mid = nums.len() / 2;

        let mut node = TreeNode::new(nums[mid]);
        node.left = Self::helper(&nums[0..mid]);
        node.right = Self::helper(&nums[mid + 1..nums.len()]);

        Some(Rc::new(RefCell::new(node)))
    }
}
