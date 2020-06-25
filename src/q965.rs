mod q965 {
    use std::borrow::Borrow;
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;

    // Definition for a binary tree node.
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

    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut q = Vec::new();
        let val = match root.as_ref() {
            Some(node) => node.as_ref().borrow().val,
            None => 0,
        };
        q.push(root);

        while let Some(node) = q.pop() {
            if let Some(node) = node {
                if node.as_ref().borrow().val != val { return false; }
                let left = node.as_ref().borrow().left.clone();
                let right = node.as_ref().borrow().right.clone();
                if left != None { q.push(left); }
                if right != None { q.push(right); }
            }
        }

        true
    }
}