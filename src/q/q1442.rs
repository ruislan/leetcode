use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        // 方法1
        // 由于数据量最大只有300，可以尝试暴力求解
        // 实际上我们可能有4个循环，也就是O(n^4)
        // AC 1028ms 1.9mb
        // let n = arr.len();
        // let mut answer = 0;
        // for i in 0..n {
        //     for j in i + 1..n {
        //         let a = (i..j).fold(0, |acc, x| acc ^ arr[x]);
        //         for k in j..n {
        //             let b = (j..=k).fold(0, |acc, x| acc ^ arr[x]);
        //             if a == b {
        //                 answer += 1;
        //             }
        //         }
        //     }
        // }
        // answer

        // 方法2
        // 很明显暴力求解中存在大量的重复xor计算
        // 所以我们可以用前缀和的方式求解每个位置的xor结果
        // 这样我们就可以求得任意两个位置之间的xor结果
        // 这里变成了O(n^3)，提升效果还是很明显
        // AC 28ms 2.1mb
        // let n = arr.len();
        // let mut prefix = vec![0; n + 1];
        // for i in 1..=n {
        //     prefix[i] = arr[i - 1] ^ prefix[i - 1];
        // }
        // let mut answer = 0;
        // for i in 0..n {
        //     for j in i + 1..n {
        //         for k in j..n {
        //             if prefix[i] ^ prefix[k + 1] == 0 {
        //                 answer += 1;
        //             }
        //         }
        //     }
        // }
        // answer

        // 方法3
        // 通过方法2，我们可以看出来，其实j不重要
        // 还要i和k之间保持xor等于0就可以了，而(i,k)中间有多少j就有多少满足条件的
        // AC 0ms 1.9mb
        let n = arr.len();
        let mut prefix = vec![0; n + 1];
        for i in 1..=n {
            prefix[i] = arr[i - 1] ^ prefix[i - 1];
        }
        let mut answer = 0;
        for i in 0..n {
            for k in i + 1..n {
                if prefix[i] ^ prefix[k + 1] == 0 {
                    answer += (k - i) as i32;
                }
            }
        }
        answer
    }
}