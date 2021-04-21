use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        // 方法1
        // 利用回溯算法，将所有的情况都罗列出来，然后利用路径记忆化来剪枝
        // 这种方式也叫动态规划
        // 这里有一个容易遗漏的点就是记忆化的时候会有一些没有进入记忆化
        // 因为我们在遇到"0"的时候就会终止搜索，所以我们如果在判断是否有"0"
        // 之前就用记忆化来返回结果会出现错误
        // 所以我们必须放在判断了"0"和结果之后再用记忆化来处理就不会有问题
        // 这里记忆化索引/后缀字符/路径都可以
        // AC 0ms 2.3mb
        use std::collections::HashMap;

        fn dfs(path: &mut Vec<String>, i: usize, s: &str, memo: &mut HashMap<String, i32>) -> i32 {
            if !path.is_empty() {
                let last = path.last().unwrap();
                if last.starts_with("0") {
                    return 0;
                }
                if last.parse::<i32>().unwrap_or(27) > 26 {
                    return 0;
                }
            }
            if i == s.len() {
                return 1;
            }
            if let Some(&count) = memo.get(&s[i..]) {
                return count;
            }

            let mut count = 0;
            path.push(s[i..i + 1].to_string());
            count += dfs(path, i + 1, s, memo);
            path.pop();

            if i + 2 <= s.len() {
                path.push(s[i..i + 2].to_string());
                count += dfs(path, i + 2, s, memo);
                path.pop();
            }

            memo.entry(s[i..].to_string()).or_insert(count);
            count
        }

        dfs(&mut Vec::new(), 0, &s, &mut HashMap::new())
    }
}
