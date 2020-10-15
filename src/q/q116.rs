#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
    pub next: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
            next: None,
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;
use crate::q::Solution;

impl Solution {
    // 此题没有Rust入口，用kotlin解决
    // connect 方法和116 重复，更名为connect_peer
    fn connect_peer(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // 方法1
        // 广度优先
        // 然后将所有子节点指向它的邻居
        // Kotlin Passed 228ms 35.4mb
        //        val queue = java.util.ArrayDeque<Node>()
        //        if (root != null) queue.addLast(root)
        //        while (queue.isNotEmpty()) {
        //            var child: Node? = null
        //            for (i in 0 until queue.size) {
        //                val node = queue.pollFirst()
        //                if (child != null) child.next = node.left
        //                if (node.left != null) {
        //                    node.left?.next = node.right
        //                    queue.addLast(node.left!!)
        //                }
        //                if (node.right != null) queue.addLast(node.right!!)
        //                child = node.right
        //            }
        //        }
        //        return root

        // 方法2
        // 不使用额外的空间
        // 采用前序遍历，这样首先遍历中间的节点，然后处理左右子节点
        // 将左子节点指向右子节点，然后右子节点指向这个子节点平级的节点的左子节点
        // Kotlin Passed 252ms 35.4mb
        //        fun preOrder(node: Node?, rightBrother: Node?) {
        //            if (node != null) {
        //                node.left?.next = node.right
        //                node.right?.next = rightBrother?.left
        //                preOrder(node.left, node.right)
        //                preOrder(node.right, rightBrother?.left)
        //            }
        //        }
        //        preOrder(root, null)
        //        return root

        // 方法3
        // 方法2的优化
        // 采用前序遍历，利用每个节点都有next的特点，直接用next来连接
        // Kotlin Passed 244ms 36.2mb
        //        fun preOrder(node: Node?) {
        //            if (node != null) {
        //                node.left?.next = node.right
        //                node.right?.next = node.next?.left
        //                preOrder(node.left)
        //                preOrder(node.right)
        //            }
        //        }
        //        preOrder(root)
        //        return root
        fn pre_order(node: Option<Rc<RefCell<TreeNode>>>) {
            if let Some(node) = node {
                if let Some(left) = node.borrow().left.clone() {
                    left.borrow_mut().next = node.borrow().right.clone();
                }
                if let Some(right) = node.borrow().right.clone() {
                    if let Some(next) = node.borrow().next.clone() {
                        right.borrow_mut().next = next.borrow().left.clone();
                    }
                }
                pre_order(node.borrow().left.clone());
                pre_order(node.borrow().right.clone());
            }
        }
        root
    }
}