use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        // 方法1
        // 暴力法
        // 将nums的两倍存储到pows数组中
        // 然后检查每一个nums[i]是否比后面的pows[j]大，其中i < j
        // Did Not Pass, 超时，时间1048ms
        // let mut pows = vec![0_i64; nums.len()];
        // let mut arr = vec![0_i64; nums.len()];
        // nums.into_iter().enumerate().for_each(|(i, x)| {
        //     arr[i] = x as i64;
        //     pows[i] = arr[i] << 1;
        // });
        //
        // let mut answer = 0;
        // let n = arr.len();
        // for i in 0..n {
        //     for j in i + 1..n {
        //         if arr[i] > pows[j] {
        //             answer += 1;
        //         }
        //     }
        // }
        // answer

        // 方法2
        // 我们假设这个数组各自的那一半是有序的，例如： 3 5 ｜ 1 2
        // 那么，3 > 1 * 2，则5肯定大于1 * 2，因为5比3还大
        // 那么，接着，3 < 2 * 2，我们再判断5，5 > 2*2，
        // 两边都没有了，返回结果
        // 那么如果这个数组很大的话，我们是不是可以一直切分成两个数组
        // 一直到每个数组只有一个数字
        // 这是不是就是归并排序了，只是归并排序多了一个检查翻转对的过程
        // 归并排序一般都采用递归处理，
        // 递归出口就是全部有序
        // 递归过程就是刚刚所说的过程，和归并的过程
        // 注意归并需要一个辅助数组
        // Passed 64ms 2.5mb
        fn merge(nums: &mut Vec<i32>, aux: &mut Vec<i32>, lo: usize, hi: usize, cnt: &mut i32) {
            if lo < hi { // 出口条件
                // 递归归并
                let mid = lo + (hi - lo) / 2;
                merge(nums, aux, lo, mid, cnt);
                merge(nums, aux, mid + 1, hi, cnt);

                // 两边已经有序
                // 开始检查
                let (mut i, mut j) = (lo, mid + 1);
                while i <= mid && j <= hi {
                    if nums[i] as i64 > (nums[j] as i64) * 2 {
                        *cnt += (mid - i) as i32 + 1; // i..mid 都比j*2大
                        j += 1;
                        continue;
                    }
                    i += 1;
                }

                // 归并两个有序数组
                for k in lo..=hi {
                    aux[k] = nums[k]; // 复制左右数组到辅助数组
                }

                let (mut i, mut j) = (lo, mid + 1);
                for k in lo..=hi {
                    if i > mid { // 左边已经取尽
                        nums[k] = aux[j];
                        j += 1;
                    } else if j > hi { // 右边已经取尽
                        nums[k] = aux[i];
                        i += 1;
                    } else if aux[i] > aux[j] { // 左边大于右边，取右边
                        nums[k] = aux[j];
                        j += 1;
                    } else { // 右边大于左边，取左边
                        nums[k] = aux[i];
                        i += 1;
                    }
                }
            }
        }

        let n = nums.len();
        if n < 2 { return 0; }
        let mut nums = nums;
        let mut aux = vec![0; n];
        let mut answer = 0;
        merge(&mut nums, &mut aux, 0, n - 1, &mut answer);
        answer
    }
}

#[test]
fn test() {
    assert_eq!(Solution::reverse_pairs(vec![]), 0);
    assert_eq!(Solution::reverse_pairs(vec![1]), 0);
    assert_eq!(Solution::reverse_pairs(vec![1, 3]), 0);
    assert_eq!(Solution::reverse_pairs(vec![3, 1]), 1);
    assert_eq!(Solution::reverse_pairs(vec![1, 3, 2, 3, 1]), 2);
    assert_eq!(Solution::reverse_pairs(vec![2, 4, 3, 5, 1]), 3);
    assert_eq!(Solution::reverse_pairs(vec![2147483647, 2147483647, 2147483647, 2147483647, 2147483647, 2147483647]), 0);
}