use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// 方法1
// init: 中序遍历存储到数组arr中,i = 0, O(n)内存不满足条件O(h)
// next: arr[i++] O(1)
// hasNext: i < arr.size() O(1)
// 方法2
// init: Stack(root)
// next: 弹出top，中序非递归进栈，直到找到最小的那个，平均O(1)，内存O(h)
// hashNext: stack是否为空 O(1)

#[derive(Default)]
struct BSTIterator {}

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self { BSTIterator::default() }

    fn next(&self) -> i32 { 0 }

    fn has_next(&self) -> bool { false }
}