use crate::q::Solution;

impl Solution {
    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        // 方法1
        // 首先统计words中每个word的最小字母频次，存储在数组a中，
        // 然后依次统计queries中每个query的最小字母频次，并与数组a中的数字进行对比
        // 只要数组a中的数字比query的数字大，结果就+1，
        // 对比的结果存储在数组res
        fn f(v: &Vec<String>) -> Vec<i32> {
            let mut res = Vec::new();
            for i in 0..v.len() {
                let mut min = (255, 0);
                for b in v[i].bytes() {
                    if min.0 > b {
                        min = (b, 1);
                    } else if min.0 == b {
                        min.1 += 1;
                    }
                }
                res.push(min.1);
            }
            res
        }

        let mut f_words = f(&words);
        f_words.sort();
        let f_queries = f(&queries);

        let mut res = Vec::new();
        for i in 0..f_queries.len() {
            let mut size = f_words.len();
            let mut base = 0_usize;
            while size > 1 {
                let half = size / 2;
                let mid = base + half;
                base = if f_words[mid] > f_queries[i] { base } else { mid };
                size -= half;
            }

            res.push(if f_words[base] == f_queries[i] { f_words.len() - base - 1 } else { f_words.len() - base - if f_words[base] < f_queries[i] { 1 } else { 0 } } as i32);
        }

        // for i in 0..f_queries.len() {
        //     let mut size = f_words.len();
        //     let mut base = 0_usize;
        //     while size > 1 {
        //         let half = size / 2;
        //         let mid = base + half;
        //         base = if f_words[mid] <= f_queries[i] { mid } else { base };
        //         size -= half;
        //     }
        //     base = if f_words[base] == f_queries[i] { base } else { base + 1 };
        //     res.push(if f_words[base] == f_queries[i] { f_words.len() - base - 1 } else { f_words.len() - base } as i32);
        // }

        // for i in 0..f_queries.len() {
        //     let base = f_words.binary_search(&f_queries[i]);
        //     match f_words.binary_search(&f_queries[i]) {
        //         Ok(base) => res.push((f_words.len() - base - 1) as i32),
        //         Err(base) => res.push((f_words.len() - base) as i32),
        //     }
        // }
        res
    }
}