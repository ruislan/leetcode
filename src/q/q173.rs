use std::cell::RefCell;
use std::rc::Rc;

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
            right: None,
        }
    }
}

// 方法1
// init: 中序遍历存储到数组arr中,i = 0, O(n)内存不满足条件O(h)
// next: stack[i++] O(1)
// hasNext: i < stack.size() O(1)
// AC 12ms 5.3mb
// #[derive(Default)]
// struct BSTIterator {
//     stack: Vec<i32>,
//     cursor: usize,
// }

// #[allow(unused)]
// impl BSTIterator {
//     fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
//         fn in_order(tree_node: Option<Rc<RefCell<TreeNode>>>, stack: &mut Vec<i32>) {
//             if let Some(mut node) = tree_node {
//                 in_order(node.borrow_mut().left.take(), stack);
//                 stack.push(node.borrow().val);
//                 in_order(node.borrow_mut().right.take(), stack);
//             }
//         }
//         let mut stack = Vec::new();
//         in_order(root, &mut stack);
//         BSTIterator { stack, cursor: 0 }
//     }
//     fn next(&mut self) -> i32 {
//         let next = self.stack[self.cursor];
//         self.cursor += 1;
//         next
//     }
//     fn has_next(&self) -> bool {
//         self.cursor < self.stack.len()
//     }
// }

// 方法2
// init: stack(root)
// next: 弹出top，中序非递归进栈，直到找到最小的那个，平均O(1)，内存O(h)
// hashNext: stack是否为空 O(1)
// AC 12ms 5mb
#[derive(Default)]
struct BSTIterator {
    stack: Vec<Option<Rc<RefCell<TreeNode>>>>,
}

#[allow(unused)]
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        BSTIterator { stack: vec![root] }
    }
    fn next(&mut self) -> i32 {
        let mut node = self.stack.pop().unwrap();
        while node.is_some() || !self.stack.is_empty() {
            while let Some(mut cur) = node {
                let left = cur.borrow_mut().left.take();
                self.stack.push(Some(cur));
                node = left;
            }
            if let Some(top) = self.stack.pop() { 
                if let Some(mut cur) = top {
                    node = cur.borrow_mut().right.take();
                    if node.is_some() { self.stack.push(node); }
                    return cur.borrow().val;
                }
            }
        }
        return -1;
    }
    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}