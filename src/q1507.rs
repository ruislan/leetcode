use crate::Solution;

impl Solution {
    pub fn reformat_date(date: String) -> String {
        // 方法1
        // 用hashmap存储day和month
        // date的格式是Day Month Year，利用split_ascii_whitespace分割开
        // hashmap中查询day和month，然后用format("{}-{}-{}",yyyy,MM,dd)的格式放置结果并返回
        "".to_string()
    }
}