use std::cell::RefCell;
use std::rc::Rc;

use crate::q::Solution;

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

impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        // 方法1
        // 直接将所有的数字提取出来，然后用hashmap存储
        // 然后判断k减去当前数字是否在hashmap中
        // Passed 4ms 2.9mb
        // let mut sets = std::collections::HashSet::new();
        // let mut nums = Vec::new();
        // let mut node = root;
        // let mut queue = std::collections::VecDeque::new();
        // queue.push_back(node);
        // while !queue.is_empty() {
        //     for _ in 0..queue.len() {
        //         if let Some(node) = queue.pop_front().unwrap() {
        //             nums.push(node.borrow().val);
        //             queue.push_back(node.borrow_mut().left.take());
        //             queue.push_back(node.borrow_mut().right.take());
        //         }
        //     }
        // }
        //
        // for n in nums {
        //     if sets.contains(&(k - n)) {
        //         return true;
        //     } else {
        //         sets.insert(n);
        //     }
        // }
        // false

        // 方法2
        // 不需要多放一个数组nums来处理，直接用sets就行处理即可
        // Passed 4ms 2.9mb
        // let mut sets = std::collections::HashSet::new();
        // let mut node = root;
        // let mut queue = std::collections::VecDeque::new();
        // queue.push_back(node);
        // while !queue.is_empty() {
        //     for _ in 0..queue.len() {
        //         if let Some(node) = queue.pop_front().unwrap() {
        //             if sets.contains(&(k - node.borrow().val)) {
        //                 return true;
        //             } else {
        //                 sets.insert(node.borrow().val);
        //             }
        //             queue.push_back(node.borrow_mut().left.take());
        //             queue.push_back(node.borrow_mut().right.take());
        //         }
        //     }
        // }
        // false

        // 方法3
        // 因为BST，所以我们用中序，这样数字就是递增的，
        // 那么如果当前数字大于K，后面的数字就没必要算了
        // Passed 4ms 3.3mb
        // fn in_order(node: Option<Rc<RefCell<TreeNode>>>, k: i32, sets: &mut std::collections::HashSet<i32>, res: &mut Vec<i32>) {
        //     if let Some(node) = node {
        //         in_order(node.borrow_mut().left.take(), k, sets, res);
        //         let val = node.borrow().val;
        //         if sets.contains(&(k - val)) {
        //             res.push(val);
        //             res.push(k - val);
        //         } else {
        //             sets.insert(val);
        //             in_order(node.borrow_mut().right.take(), k, sets, res);
        //         }
        //     }
        // }
        // let mut res = Vec::new();
        // in_order(root, k, &mut std::collections::HashSet::new(), &mut res);
        // !res.is_empty()

        // 方法4
        // 因为BST，先中序遍历放进数组成为有序数组，问题就变成了有序数组求2数字和
        // 因为有序数组，所以可以使用二分查找，如果k - nums[i] 能够被二分查找到且不为自己，
        // 说明存在数字
        // Passed 4ms 3.3mb
        // fn in_order(node: Option<Rc<RefCell<TreeNode>>>, nums: &mut Vec<i32>) {
        //     if let Some(node) = node {
        //         in_order(node.borrow_mut().left.take(), nums);
        //         nums.push(node.borrow().val);
        //         in_order(node.borrow_mut().right.take(), nums);
        //     }
        // }
        // let mut nums = Vec::new();
        // in_order(root, &mut nums);
        // for i in 0..nums.len() {
        //     match nums.binary_search(&(k - nums[i])) {
        //         Ok(j) => if i != j { return true; }
        //         _ => ()
        //     }
        // }
        // false

        // 方法5
        // 双指针，如果l + r 大于k，则r -= 1，如果小于k，则l += 1
        fn in_order(node: Option<Rc<RefCell<TreeNode>>>, nums: &mut Vec<i32>) {
            if let Some(node) = node {
                in_order(node.borrow_mut().left.take(), nums);
                nums.push(node.borrow().val);
                in_order(node.borrow_mut().right.take(), nums);
            }
        }
        let mut nums = Vec::new();
        in_order(root, &mut nums);
        let (mut le, mut ri) = (0, nums.len().saturating_sub(1));
        while le < ri {
            let sum = nums[le] + nums[ri];
            match sum.cmp(&k) {
                std::cmp::Ordering::Less => le += 1,
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Greater => ri -= 1,
            }
        }
        false
    }
}