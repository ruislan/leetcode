use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        // 方法1
        // 直接将dominoes[i]按字典顺序放入hashmap
        // 进行统计即可
        // Passed 8ms 4.2mb
        // let mut freq = std::collections::HashMap::new();
        // for mut dom in dominoes {
        //     dom.sort_unstable();
        //     *freq.entry(dom).or_insert(0) += 1;
        // }
        // freq.into_iter().map(|(v, x)| {
        //     let mut i = 1;
        //     let mut sum = 0;
        //     while i <= x {
        //         sum += i;
        //         i += 1;
        //     }
        //     sum as i32
        // }).sum::<i32>()


        // 方法2
        // 用等差公式
        // Passed 8ms 4.2mb
        // let mut freq = std::collections::HashMap::new();
        // for mut dom in dominoes {
        //     dom.sort_unstable();
        //     *freq.entry(dom).or_insert(0) += 1;
        // }
        // freq.into_iter().map(|(v, x)| (x - 1) * x / 2).sum::<i32>()

        // 方法3
        // rust的hash略慢，所以我们自制一个小hash
        // 组合dominoes[i]的数字，因为范围是[1,9]，那么最大就是99
        // Passed 0ms 4.1mb
        let mut freq = vec![0; 100];
        for dom in dominoes {
            freq[(dom[0].max(dom[1]) * 10 + dom[0].min(dom[1])) as usize] += 1;
        }
        freq.into_iter().map(|x| (x - 1) * x / 2).sum::<i32>()
    }
}