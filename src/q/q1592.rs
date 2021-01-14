use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn reorder_spaces(text: String) -> String {
        // 方法1
        // 统计空格数和单词数
        // 根据 空格数 / (单词数 - 1) 得到相邻单词的空格数
        // 最后组合成字符串即可
        // Passed 0ms 2.1mb
        // let mut whitespaces = text.chars().filter(|&ch| ch == ' ').count();
        // let mut words = text.split_ascii_whitespace().collect::<Vec<&str>>();
        // let mut answer = String::new();
        // let span = if words.len() > 1 { whitespaces / (words.len() - 1) } else { whitespaces + 1 };
        // for word in words {
        //     answer.push_str(word);
        //     if whitespaces >= span {
        //         answer.push_str(&" ".repeat(span));
        //         whitespaces -= span;
        //     }
        // }
        // if whitespaces > 0 { answer.push_str(&" ".repeat(whitespaces)); }
        // answer

        // 方法2
        // 再次简化方法1
        // Passed 0ms 2.1mb
        let mut whitespaces = text.chars().filter(|&ch| ch == ' ').count();
        let mut words = text.split_ascii_whitespace().collect::<Vec<&str>>();
        if words.len() == 1 { return words[0].to_string() + &" ".repeat(whitespaces); }
        let (span, remain) = (whitespaces / (words.len() - 1), whitespaces % (words.len() - 1));
        words.join(&" ".repeat(span)) + &" ".repeat(remain)
    }
}