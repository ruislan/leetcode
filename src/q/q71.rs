use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn simplify_path(path: String) -> String {
        // 方法1
        // 利用栈来处理路径的组合
        // 迭代path，用'/'分割，然后检查每个split[i]，
        // 清理掉每个split[i]的'/'然后再次迭代
        // 如果是'.'则不管，如果是'..'则出栈，其他的都入栈
        // AC 0ms 2.1mb 256/256
        let mut uri: Vec<&str> = Vec::new();
        path.split('/')
            .filter(|&s| s != "." && !s.is_empty())
            .for_each(|s| {
                if s == ".." {
                    uri.pop();
                } else {
                    uri.push(s);
                }
            });
        if uri.is_empty() {
            String::from('/')
        } else {
            uri.into_iter()
                .fold(String::new(), |acc, s| acc + "/" + s)
        }
    }
}