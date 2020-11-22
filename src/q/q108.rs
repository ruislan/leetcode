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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        // 方法1
        // 因为输入数组是无重复数字的升序数组，我们同时也知道BST的特点就是前序遍历出来就是升序数组
        // 那么我们来一个一个的子节点放上去看看，
        // [1]，
        //             1
        // [1,2]，
        //             2
        //           1
        // [1,2,3]
        //             2
        //           1   3
        // [1,2,3,4]
        //             3
        //           2   4
        //         1
        // [1,2,3,4,5]
        //             3
        //           2   5
        //          1   4
        // [1,2,3,4,5,6]
        //             4
        //           2   6
        //          1 3 5
        // 到这里我们基本看出规律了，中间那个节点就是父节点，左右都是子节点
        // 例如[1,2,3,4,5,6]中间就是arr[3] = 4，也就是4就是父节点，而[1,2,3]和[5,6]就分别是左右子节点（树）
        // 那么因为左右还是超过长度1的数组，所以我们可以再次找到中间节点，
        // 先找左边的[1,2,3]，arr[1] = 2，2是父节点，而左右是[1],[3]，不用再继续了
        // 再找右边的[5,6]，arr[1] = 6, 6是父节点，而左边是[5]，不用再继续了
        // 所以，
        // 方法1
        // 上面总结的规律看来，递归是最好的方法，写一个递归函数build，参数为父节点和数组
        // 缩小范围：我们查找数组的中间节点，并让这个中间节点作为父节点，他的左边子数组和右边子数组分别作为
        //         独立数组和这个父节点放入递归build中
        // 基线：   如果中间有节点，比父节点大就放在父节点的右子节点处，比父节点小就放在左子节点处
        // Passed 0ms 2.8mb
        // fn build_bst(nums: &[i32], mut parent: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        //     if !nums.is_empty() {
        //         let mid = nums.len() / 2;
        //         let node = Some(Rc::new(RefCell::new(TreeNode { val: nums[mid], left: None, right: None })));
        //         match parent.as_mut() {
        //             None => parent = node.clone(),
        //             Some(parent) => {
        //                 let mut parent = parent.borrow_mut();
        //                 if nums[mid] > parent.val { parent.right = node.clone(); } else { parent.left = node.clone(); }
        //             }
        //         }
        //         build_bst(&nums[..mid], node.clone());
        //         build_bst(&nums[mid + 1..], node);
        //     }
        //     parent
        // }
        // build_bst(&nums, None)

        // 方法2
        // 改进一下方法1，方法1略显臃肿，我们其实知道左边是左子节点，右边是右子节点，
        // 那么我们其实不用去判断当前节点的值和父节点的比对
        // 我们只需要不停的返回最高的那个节点就可以了
        // Passed 4ms 3mb
        // if nums.is_empty() { return None; }
        // let mid = nums.len() / 2;
        // Some(Rc::new(RefCell::new(TreeNode {
        //     val: nums[mid],
        //     left: Self::sorted_array_to_bst(nums[..mid].to_vec()),
        //     right: Self::sorted_array_to_bst(nums[mid + 1..].to_vec()),
        // })))

        // 方法3
        // 不停拷贝数组是有点浪费空间，我们还是写一个递归函数来解决吧
        fn build_bst(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if nums.is_empty() { return None; }
            let mid = nums.len() / 2;
            Some(Rc::new(RefCell::new(TreeNode {
                val: nums[mid],
                left: build_bst(&nums[..mid]),
                right: build_bst(&nums[mid + 1..]),
            })))
        }
        build_bst(&nums)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::sorted_array_to_bst(vec![1]), Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: None,
    }))));
    assert_eq!(Solution::sorted_array_to_bst(vec![1, 2, 3]), Some(Rc::new(RefCell::new(TreeNode {
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
    }))));
}