use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        // 方法1
        // 设置hashset，用来存放各个单词（word_list中单词不重复）
        // 设置待处理单词列表words = [begin_word]
        // 设置步骤计数answer
        // 当待处理单词列表不为空 并且 hashset不为空时，
        // 依次取出待处理单词列表的单词，
        //    计数answer += 1
        //    在hashmap中找出只变化了一个单词的所有词
        //        找出只变化了一个的词用zip连接两个单词（因为长度一样），然后比较两个单词的相异个数即可
        //    将这些词从hashset中取出
        //    比较这些单词，如果是end_word，返回结果
        //    将这些词存入待处理单词列表
        // 返回0
        // Passed 220ms 2.3mb
        // let mut sets: std::collections::HashSet<String> = word_list.into_iter().map(|x| x).collect();
        // if !sets.contains(&end_word) { return 0; }
        // let mut answer = 1;
        // let mut words = std::collections::VecDeque::new();
        // words.push_back(begin_word);
        // while !words.is_empty() && !sets.is_empty() {
        //     answer += 1;
        //     for _ in 0..words.len() {
        //         if let Some(word) = words.pop_front() {
        //             let mut next_words = Vec::new();
        //             for next in sets.iter() {
        //                 if word.chars().zip(next.chars()).filter(|(a, b)| a != b).count() == 1 {
        //                     next_words.push(next.clone());
        //                 }
        //             }
        //             for next in next_words {
        //                 if next == end_word {
        //                     return answer;
        //                 }
        //                 sets.remove(&next);
        //                 words.push_back(next);
        //             }
        //         }
        //     }
        // }
        // 0

        // 方法2
        // 思路都差不多，但是方法1有点慢，我们要优化一下
        // 主要优化的地方是在字符串对比上，我们如果每次都拆开来对比，会增加不少操作
        // 我们先全部拆开，然后通过替换某个字符，然后用替换后的去查找set中是否存在，
        // 这样就能减少很多拆分操作
        // Passed 44ms 2.5mb （感觉还能优化，今天没时间了，后面有时间再想想怎么优化）
        let mut sets: std::collections::HashSet<Vec<u8>> = word_list.into_iter().map(|x| x.into_bytes()).collect();
        let end_word = end_word.into_bytes();
        let mut answer = 1;
        let mut words = std::collections::VecDeque::new();
        words.push_back(begin_word.into_bytes());
        while !words.is_empty() && !sets.is_empty() {
            answer += 1;
            for _ in 0..words.len() {
                let mut word = words.pop_front().unwrap();
                for i in 0..word.len() {
                    let origin = word[i];
                    for c in b'a'..=b'z' {
                        word[i] = c;
                        if let Some(next) = sets.take(&word) {
                            if next == end_word { return answer; }
                            words.push_back(next);
                        }
                    }
                    word[i] = origin;
                }
            }
        }
        0
    }
}

#[test]
fn test() {
    use crate::slice_to_string_vec;

    assert_eq!(Solution::ladder_length(
        String::from("hit"), String::from("cog"),
        slice_to_string_vec(&["hot", "dot", "dog", "lot", "log", "cog"]),
    ), 5);
    assert_eq!(Solution::ladder_length(
        String::from("hit"), String::from("cog"),
        slice_to_string_vec(&["hot", "dot", "dog", "lot", "log"]),
    ), 0);
}