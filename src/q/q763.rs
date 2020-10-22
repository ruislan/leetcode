use crate::q::Solution;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        // 方法1
        // 线性遍历S，将字符最后出现的索引存储和字符的映射bucket中
        // 设置分区p = 0，结果集res，
        // 线性遍历S
        //    令pos = bucket[s[i]]
        //    如果i == pos 且 bucket[s[i]] >= p:
        //       i 存入结果集
        //    如果pos > p:
        //       p = bucket[s[i]]
        // 最后将res从后向前，减去前面的值，这样得出个数
        // 然后res[0]的个数加1
        // 返回res
        // Passed 0ms 1.9mb
        // let mut bucket = vec![0; 26];
        // s.bytes().enumerate().for_each(|(i, ch)| bucket[ch as usize - 97] = i);
        // let mut res = Vec::new();
        // let mut p = 0;
        // s.bytes().enumerate().for_each(|(i, ch)| {
        //     let pos = bucket[ch as usize - 97];
        //     if pos == i && pos >= p { res.push(i as i32); }
        //     if pos > p { p = pos; }
        // });
        // (1..res.len()).rev().for_each(|i| res[i] = res[i] - res[i - 1]);
        // res[0] += 1;
        // res

        // 方法2
        // 方法1的改进，最后来向前计算个数有点累赘，所以我们可以利用双指针直接在遍历的时候就计算好结果
        let mut bucket = vec![0; 26];
        s.bytes().enumerate().for_each(|(i, ch)| bucket[ch as usize - 97] = i);
        let mut res = Vec::new();
        let (mut start, mut end) = (0, 0);
        s.bytes().enumerate().for_each(|(i, ch)| {
            end = end.max(bucket[ch as usize - 97]);
            if end == i {
                res.push((end - start + 1) as i32);
                start = i + 1;
            }
        });
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::partition_labels("a".to_string()), vec![1]);
    assert_eq!(Solution::partition_labels("abc".to_string()), vec![1, 1, 1]);
    assert_eq!(Solution::partition_labels("abac".to_string()), vec![3, 1]);
    assert_eq!(Solution::partition_labels("abca".to_string()), vec![4]);
    assert_eq!(Solution::partition_labels("abaccbdeffed".to_string()), vec![6, 6]);
    assert_eq!(Solution::partition_labels("ababcbacadefegdehijhklij".to_string()), vec![9, 7, 8]);
}