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
    fn postorder(root: Node) -> Vec<i32> {
        // 方法1
        // 递归，so easy
        // kotlin code Passed 268ms 37.4mb
        //        fun travel(node: Node?, values: MutableList<Int>) {
        //            if (node != null) {
        //                node.children.forEach {
        //                    travel(it, values)
        //                }
        //                values.add(node.`val`)
        //            }
        //        }
        //
        //        val res = mutableListOf<Int>()
        //        travel(root, res)
        //        return res

        // 方法2
        // 迭代，观察后序输出顺序，可以看出来，
        // 这完全就是深度优先搜索（从右子节点开始），
        // 最后结果数组再反转即可
        // kotlin code  Passed 324ms 37.4mb
        //        val stack = java.util.Stack<Node>()
        //        val res = mutableListOf<Int>()
        //        if (root != null) stack.push(root)
        //        while (stack.isNotEmpty()) {
        //            val node = stack.pop();
        //            res.add(node.`val`)
        //            node.children.forEach { if (it != null) stack.push(it) }
        //        }
        //        return res.reversed()

        let mut res = Vec::new();
        let mut stack = vec![root];
        while stack.is_empty() {
            if let Some(node) = stack.pop() {
                res.push(node.val);
                node.children.into_iter().for_each(|x| stack.push(x));
            }
        }
        res.reverse();
        res
    }
}