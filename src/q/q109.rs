#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

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

use std::rc::Rc;
use std::cell::RefCell;
use crate::q::Solution;

impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        // 这个题吧是前面108那个题的升级版本
        // 单链表存储了一个按照升序排列一串数字，然后组合成平衡二叉搜索树

        // 方法1
        // 因为题目没有限制，这里我们可以简单处理
        // 遍历链表放在有序数组中，然后通过108的方法就能生成平衡二叉搜索树

        // 方法2
        // 链表的特点，只能逐个获得值，那么就是要逐个的去添加子节点
        // 这就涉及到在添加新的节点的时候要通过旋转方式来平衡二叉树
        // 这里我们要写一个添加新节点的方法，参数是当前节点，和要添加的新的值
        // 通过迭代链表，逐个将节点添加进去。
        None
    }
}