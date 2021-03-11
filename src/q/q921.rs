use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        // 方法1
        // 这题其实刚刚好够中等难度的边吧，算是中等里面的简单题
        // ‘(’必然要应对一个')'才不需要修改
        // 所以，我们其实把这个题转变成了一个遇到')'，然后前面是'('的就消掉
        // 类似这样需要用到前面知识的题，一般都和栈有关系
        // 例如“()))((”，
        // 1是"("，进栈，栈["("]
        // 2是“)”，看栈顶是"("，那么出栈，栈[]
        // 3是")"，看栈顶是空的，进栈，栈[")"]
        // 4是")"，看栈顶是")"，无法消除，进栈，栈[")",")"]
        // 5是"("，进栈，栈["),),("]
        // 6是"("，进栈，栈["),),(,("]
        // 最后栈内的都是无法消除的，所以我们都要干掉他们才符合条件
        // AC 0ms 1.9mb
        let mut stack = Vec::new();
        for ch in s.chars() {
            if ch == '(' {
                stack.push(ch);
            } else {
                if !stack.is_empty() && stack[stack.len() - 1] == '(' {
                    stack.pop();
                } else {
                    stack.push(ch);
                }
            }
        }
        stack.len() as i32
    }
}