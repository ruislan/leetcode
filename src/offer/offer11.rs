use crate::offer::Solution;

impl Solution {
    // 本题同q154
    pub fn min_array(numbers: Vec<i32>) -> i32 {
        // 方法1
        // 排序，然后取第一个
        // Passed 0ms 2.1mb
        // let mut numbers = numbers;
        // numbers.sort();
        // if numbers.is_empty() { 0 } else { numbers[0] }

        // 方法2
        // 系统函数
        // numbers.into_iter().min().unwrap_or(0)

        // 方法3
        // 线性查找，O(n)，以第一个为最小值，如果比这个值小的，就替代，直到最后一个
        // let mut min = if numbers.is_empty() { 0 } else { numbers[0] };
        // numbers.into_iter().for_each(|n| if min > n { min = n; });
        // min

        // 方法3
        // 我们可以不断裁剪数组，最后来锁定这个数字，一定会有一个最小数字，
        // 所以这里我们定义l,r,m分别代表左，右，中，初始化l = 0 , r = len - 1,
        // 首先计算m = (l + r) / 2，然后 取出l, r, m的值
        // 如果vm < vr，则说明中间比右边的值都小（升序嘛），所有最小值在左边，剪裁右边
        // 如果vm > vr, 则说明右边的值比左边的都小，所以最小值在右边，剪裁左边
        // 剩下vm == vr，可能在左边也可能在右边，（如22202,20222）那么我们直接剪裁vr这个点
        // 循环直到找出最小值
        let (mut le, mut ri) = (0, numbers.len() - 1);
        while le < ri {
            let mid = (le + ri) / 2;
            if numbers[mid] < numbers[ri] { ri = mid; } else if numbers[mid] > numbers[ri] { le = mid + 1; } else { ri -= 1; }
        }
        numbers[le]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_array(vec![3, 4, 5, 1, 2]), 1);
    assert_eq!(Solution::min_array(vec![2, 2, 2, 0, 1]), 0);
    assert_eq!(Solution::min_array(vec![3, 4, 5]), 3);
    assert_eq!(Solution::min_array(vec![5, 4, 3]), 3);
    assert_eq!(Solution::min_array(vec![1]), 1);
}