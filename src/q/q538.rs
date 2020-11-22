use std::cell::RefCell;
use std::rc::Rc;

use crate::q::Solution;

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

#[allow(unused)]
impl Solution {
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // 方法1
        // 关键点，二叉搜索树，每个节点的值是原来的节点值加上所有比它大的节点值之和
        // 因为bst的原因，所以比当前节点大的值都在右边，也即是要先处理最右边的，然后再处理中间的，再处理左边的
        // 想想中序遍历是左中右，而现在要变成右中左，做个镜像就可以了，遍历的时候需要一个累加值，一直累加着
        // Passed 4ms 2.8mb
        fn in_order_rev(node: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
            if let Some(node) = node {
                in_order_rev(node.borrow().right.clone(), sum);
                node.borrow_mut().val += *sum;
                *sum = node.borrow().val;
                in_order_rev(node.borrow().left.clone(), sum)
            }
        }
        in_order_rev(root.clone(), &mut 0);
        root

        // 方法2
        // 其实没有方法2，就纯粹是想看看我多一个数组会不会时间长点，所以试了一下，嗯，结果很喜人
        // 将root按照中序遍历的镜像读取到一个数组中，然后处理数组的倒序累加
        // 然后再遍历赋值
        // Passed 4ms 2.9mb
        // fn in_order_rev(node: Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
        //     if let Some(node) = node {
        //         in_order_rev(node.borrow().right.clone(), vals);
        //         vals.push(node.borrow().val);
        //         in_order_rev(node.borrow().left.clone(), vals);
        //     }
        // }
        // fn assign(node: Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
        //     if let Some(node) = node {
        //         assign(node.borrow().left.clone(), vals);
        //         node.borrow_mut().val = vals.pop().unwrap();
        //         assign(node.borrow().right.clone(), vals);
        //     }
        // }
        // let mut vals = Vec::new();
        // in_order_rev(root.clone(), &mut vals);
        // (1..vals.len()).for_each(|i| vals[i] += vals[i - 1]);
        // assign(root.clone(), &mut vals);
        // root
    }
}

#[test]
fn test() {
    assert_eq!(Solution::convert_bst(
        Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })))
    ),
               Some(Rc::new(RefCell::new(TreeNode {
                   val: 5,
                   left: Some(Rc::new(RefCell::new(TreeNode {
                       val: 6,
                       left: None,
                       right: None,
                   }))),
                   right: Some(Rc::new(RefCell::new(TreeNode {
                       val: 3,
                       left: None,
                       right: None,
                   }))),
               })))
    )
}