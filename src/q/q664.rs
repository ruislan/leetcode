use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        // 方法1
        // 这道题一开始是真的没思路，因为就是题目描述的情况下，就是人脑都找最优解都很费，
        // 例子太简单了，看不出什么，所以要扩展了一下aba这么一个例子为abccbaa这样，
        // 我能想到第一个就是尽可能的覆盖最多的区间字符，axxxxaa，
        // 这样子，这里有个分界点就出来了，中间留下的是bccb还是bccba呢，
        // 肯定希望留下的是bccb,也就是说其实axxxxa就可以了，最后那个a完全是附带送的
        // 那么bccb是不是就可以处理成bxxb，就留下cc了
        // 这样看起来有点栈或者递归的感觉啊，看来有点门道了
        // 然后我们从例子回到普遍情况，我们知道第一个a，但是我们不知道后面是不是有a，
        // 我们希望是有a的，这样，我们可以做成axxxa的情况，那如果后面还有a呢，
        // 例如axxxayyya，那我们是选择中间那个a还是后面那个a？
        // 这里是不是要看中间xxx的最小打印次数，如果xxxayyy比xxx要小，那自然我们就选择长的
        // 那么中间的xxx(或者xxxayyy)也会用同样的规则来处理，
        // 所以选择和当前要打印字符的相同字符成了一个要考虑的点
        // 那么，这个点我们不知道在哪里，所以我们用k in i + 1..=j的范围来找，
        // 如果找到了，我们就从k点分割成i+1..k-1和k..=j这两个范围，
        // 然后再递归用同样的规则处理。
        // 那么现在就有个大体的框架了
        // 首先我们有个递归方法print_range表示每次打印，然后有i和j表示当前打印的范围
        // 如果i > j，说明我们打印了一个空串，意思就是不用打印
        // 然后我们尝试打印i+1..=j，得到打印次数连同我们当前这个字符1次，作为我们第一个参考的打印数量
        // 然后我们开始从i点开始分割字符串，当s[i] == s[k]的时候
        // 得到每次分割的打印次数，再和参考数量比较，选择最小的那个
        // 这样直到最后，我们就得到了最小的打印次数
        // AC 8ms 2mb
        fn print_range(s: &Vec<u8>, i: usize, j: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
            if i > j { return 0; }
            if memo[i][j] > 0 { return memo[i][j]; }
            let mut count = 1 + print_range(s, i + 1, j, memo);
            for k in i + 1..=j {
                if s[i] == s[k] {
                    count = count.min(print_range(s, i + 1, k - 1, memo) + print_range(s, k, j, memo));
                }
            }
            memo[i][j] = count;
            count
        }

        let n = s.len();
        print_range(&s.into_bytes(), 0, n - 1, &mut vec![vec![0; n]; n])
    }
}