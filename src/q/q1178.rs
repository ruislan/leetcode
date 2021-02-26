use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        // 方法1
        // 这题的难度在于优化，第一眼看下来就知道一种最简单的方式，
        // 就是统计每个word的字母出现情况，统计每个puzzle的字母出现情况和它的首字母
        // 然后依次比较，这将会成为O(len(words) * len(puzzles) * 7 * len(word))
        // 在数据量不大的情况下，测试一些案例将不会有问题
        // 也就是说这个方向没有问题，接下来我们来优化一下它
        // 我们知道word都是小写字母，而我们也只需要知道字母有无，所以我们可以将字母压缩到i32数字中
        // 这样一来，我们可以将所有的字母都全部压缩成二进制数字，这样两个二进制数字相或得到的结果
        // 就是puzzle，那么表示word被puzzle包含，这样,7*len(word)就可以简化成1，
        // 时间复杂度就成了O(len(words) * len(puzzles))，但是很遗憾，提交之后，我们还是
        // 超时
        // let mut words_freq = Vec::new();
        // for (i, word) in words.iter().enumerate() {
        //     let mut mask = 0_i32;
        //     let mut bits = vec![false; 26];
        //     for ch in word.bytes() {
        //         mask |= (1 << (ch - 97));
        //         bits[(ch - 97) as usize] = true;
        //     }
        //     if mask.count_ones() <= 7 {
        //         words_freq.push((bits, mask));
        //     }
        // }
        //
        // let mut answer = Vec::new();
        // for puzzle in puzzles {
        //     let puzzle = puzzle.into_bytes();
        //     let mut mask = 0_i32;
        //     for i in 0..7 {
        //         mask |= 1 << (puzzle[i] - 97);
        //     }
        //     let mut count =
        //     words_freq.iter().filter(|x| {
        //         x.0[(puzzle[0] - 97) as usize] && x.1 | mask == mask
        //     }).count() as i32;
        //     answer.push(count);
        // }
        // answer

        // 方法2
        // 优化方法1
        // 看来我们要将O(n^2)再次降低，
        // 假设word的字母都被puzzle所包含，那么他们的二进制中，
        // word的1的位置，puzzle一定也有1，而假设我们能够想一种方法
        // 能够得到puzzle的二进制（除首字母）的1的全部排列，
        // 然后每个排列并上首字母去检索word的二进制数字，两个如果相等，那么说明word就被puzzle所包含
        // 而如果我们将word的二进制的数值存储到hashmap中，这样一来，我们就可以减少检索为O(1)
        // 那么我们的复杂度就降低到O(n*m)，而这个m远远小于n的
        // 而要得到puzzle的二进制（除首字母）的1的全部排列，需要一个技巧，那就是
        // sub_mask = (sub_mask - 1) & mask
        // 当sub_mask = -1的时候，所有的二进制位都是1，所以再 & mask就等于mask自己，这时候我们就无需检查了
        // AC 32ms 10.1mb
        let mut words_freq = std::collections::HashMap::new();
        for word in words.iter() {
            let mut mask = 0_i32;
            for ch in word.bytes() {
                mask |= (1 << (ch - 97));
            }
            if mask.count_ones() <= 7 {
                *words_freq.entry(mask).or_insert(0) += 1;
            }
        }

        let mut answer = Vec::new();
        for puzzle in puzzles {
            let puzzle = puzzle.into_bytes();
            let mut mask = 0_i32;
            let mut init = 1 << (puzzle[0] - 97);
            for i in 1..7 {
                mask |= 1 << (puzzle[i] - 97);
            }
            let mut count = 0;
            let mut subset = mask;
            loop {
                if let Some(&freq) = words_freq.get(&(subset | init)) {
                    count += freq;
                }
                subset = (subset - 1) & mask; // 这个技巧要学会哟
                if subset == mask {
                    break;
                }
            }
            answer.push(count);
        }
        answer
    }
}