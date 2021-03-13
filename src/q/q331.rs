use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        // 方法1
        // 这个题在处理的时候会发现我们会往回看前面的元素来左判断，
        // 那么这种情况就要用到栈了
        // 首先排除无用的字符','，把其他字符都组织起来
        // 然后非"#"，那么就是数字，可以直接入栈
        // 如果是“#”，那么循环检查栈顶，
        //     如果也是“#”，则弹出2个元素
        //     如果在正常情况下，是数字+“#”，那么第二个元素一定是数字，
        //     不是数字，或者没有办法弹出了，那肯定就是错误的，直接返回false即可
        //     直到不能弹出了，也就是栈为空了或者栈顶不是“#”，退出循环
        // 因为我们弹出了两个数字，现在还剩一个“#”，我们要把它加入进去
        // 如果是一颗正常的树，那么到最后一定只剩下一个“#”，因为所有的元素都被弹出了
        // 而空树就是空节点"#"，所以最后检查是否只有一个元素在栈中，且为“#”即可
        // AC 0ms 2.2mb
        // let mut stack = Vec::new();
        // let preorder: Vec<&str> = preorder.split_terminator(",").collect();
        // for s in preorder.into_iter() {
        //     if s != "#" {
        //         stack.push(s);
        //         continue;
        //     }
        //     while !stack.is_empty() && stack[stack.len() - 1] == "#" {
        //         stack.pop();
        //         if let Some(x) = stack.pop() {
        //             if x == "#" { return false; }
        //         } else {
        //             return false;
        //         }
        //     }
        //     stack.push("#");
        // }
        // stack.len() == 1 && stack[0] == "#"

        // 方法2
        // 根据方法1，我们知道，其实不需要知道内部具体元素是多少，
        // 只需要和“#”打交道就行了
        // 我们来考虑最基本的树的3种合法的情况，
        // 1：空树，那么，“#”
        // 2：只有一个节点，那么，"n # #"
        // 3：有一个父节点一个子节点，那么"n n # # #"， 或者 "n # n # #"
        // 通过最小的树单元，我们知道了，一颗合法的树，#的数量，必然比n的数量多一个
        // 那么我们完全可以统计node和#的数量，但是我们也不能单纯的统计总数，
        // 因为有可能不合法是发生在某个子树上的，如果直接统计总数，就会忽略这种情况
        // 那么，我们还是来看基本单元，
        // 1：“#”，#：0->-1
        // 2："n # #"，n：0->1，#：1->0，#:0->-1，
        // 3.1：“n n # # #”，n：0->1，n：1->2，#：2->1，#：1->0，#：0->-1
        // 3.2：“n # n # #”，n：0->1，#：1->0，n：0->1，#：1->0，#：0->-1
        // 我们发现，在中间的过程中，不会出现节点低于0的情况，除非在最后一个位置
        // 所以当不为最后一个位置的节点数出现小于0的情况，直接就可以返回不合法了
        // 最后再检查节点是否为—1即可
        // AC 0ms 2mb
        let mut nodes = 0;
        let preorder: Vec<&str> = preorder.split_terminator(",").collect();
        for i in 0..preorder.len() {
            if preorder[i] != "#" {
                nodes += 1;
            } else {
                nodes -= 1;
            }
            if i < preorder.len() - 1 && nodes < 0 { return false; }
        }
        nodes == -1
    }
}