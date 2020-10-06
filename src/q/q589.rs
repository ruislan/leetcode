use crate::q::Solution;

#[derive(Debug, PartialEq, Eq)]
struct Node {
    val: i32,
    children: Vec<Node>,
}

impl Node {
    #[inline]
    pub fn new(val: i32) -> Self {
        Node {
            val,
            children: Vec::new(),
        }
    }
}

impl Solution {
    // 本题没有Rust入口，所以用Kotlin解决
    fn preorder(root: Node) -> Vec<i32> {
        // 方法1
        // 递归
        // kotlin Passed 268ms 36.1mb
        // kotlin code
        //     fun preorder(root: Node?): List<Int> {
        //         var ints = mutableListOf<Int>()
        //         pre_order(root, ints)
        //         return ints
        //     }
        //
        //     fun pre_order(node:Node?, ints:MutableList<Int>) {
        //         if (node != null) {
        //             ints.add(node.`val`)
        //             node.children.forEach {
        //                 pre_order(it, ints)
        //             }
        //         }
        //     }
        //

        // 方法2
        // 前中后序如果不用递归，那么就可以用栈，
        // 前序记住只需要将子节点以反序放入栈即可
        // kotlin Passed 276ms 36.5mb
        // kotlin code
        //     fun preorder(root: Node?): List<Int> {
        //         val ints = mutableListOf<Int>()
        //         val stack = java.util.Stack<Node?>()
        //         stack.push(root)
        //         while (stack.isNotEmpty()) {
        //             val n = stack.pop()
        //             if (n != null) {
        //                 ints.add(n.`val`)
        //                 n.children.reversed().forEach {
        //                     stack.push(it)
        //                 }
        //             }
        //         }
        //         return ints
        //     }
        let mut res = Vec::new();
        let mut stack = vec![root];
        while stack.is_empty() {
            if let Some(node) = stack.pop() {
                res.push(node.val);
                node.children.into_iter().rev().for_each(|x| stack.push(x));
            }
        }
        res
    }
}