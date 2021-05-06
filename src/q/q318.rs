use crate::q::Solution;

#[allow(unused)]
impl Solution {
    // 与q1464方法名相同，这里改为max_product_words
    pub fn max_product_words(words: Vec<String>) -> i32 {
        // 方法1
        // 由于字符都是小写的，所以可以放入26位数组里面
        // 然后比较两个数组，如果有相交的，则直接不要
        // 否则就要看哪个更大
        // AC 32ms 2.1mb
        // let n = words.len();
        // let mut v = vec![vec![false; 26]; n];
        // for i in 0..n {
        //     words[i].bytes().for_each(|x| {
        //         v[i][x as usize - 97] = true;
        //     });
        // }
        //
        // let mut answer = 0;
        // for i in 0..n {
        //     let a = &v[i];
        //     for j in i + 1..n {
        //         let b = &v[j];
        //         if a.iter().zip(b.iter()).find(|(x, y)| (**x || **y) && (x == y)).is_none() {
        //             answer = answer.max(words[i].len() as i32 * words[j].len() as i32);
        //         }
        //     }
        // }
        // answer

        // 方法2
        // 由于只有26个小写字符，所以我们可以存储到32位i32中
        // 然后利用a & b 的方式，如果结果是0，那么必然不相交
        // AC 8ms 2.5mb
        let n = words.len();
        let mut v = vec![0; n];
        for i in 0..n {
            words[i].bytes().for_each(|x| {
                let offset = x - 97;
                v[i] = (1 << offset) | v[i];
            });
        }

        let mut answer = 0;
        for i in 0..n {
            for j in i + 1..n {
                if v[i] & v[j] == 0 {
                    answer = answer.max(words[i].len() as i32 * words[j].len() as i32);
                }
            }
        }
        answer
    }
}