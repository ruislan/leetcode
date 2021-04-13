
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(unused)]
impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;
use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        // 方法1
        // 就中序遍历找到第k个即可
        // 当然，也可以不用递归的方式，这个就作为复习的时候的小挑战吧
        // AC 0ms 2.9mb
        fn in_order(node: Option<Rc<RefCell<TreeNode>>>, k: &mut i32, answer: &mut i32) {
            if let Some(mut node) = node {
                in_order(node.borrow_mut().left.take(), k, answer);
                *k -= 1;
                if *k == 0 { *answer = node.borrow().val; }
                else if *k > 0 {
                    in_order(node.borrow_mut().right.take(), k, answer);
                }
            }
        }
        let mut answer = 0; 
        let mut k = k;
        in_order(root, &mut k, &mut answer);
        answer
    }
}