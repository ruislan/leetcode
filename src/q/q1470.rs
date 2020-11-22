use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        // 方法1
        // 构建数组res，然后迭代范围0..n
        // 然后再res.push(nums[i])和nums[i + n]
        // 输出res
        // Passed 0-4ms 2mb
        // let n = n as usize;
        // let mut res = Vec::with_capacity(2 * n);
        // (0..n).for_each(|i| {
        //     res.push(nums[i]);
        //     res.push(nums[i + n]);
        // });
        // res

        // 方法2
        // 构建2n长的数组res，然后迭代0..2n每个元素为i
        // 设置x为0..2n / 2：
        // x 为even，则放入nums[x]
        // x 为odd，则放入nums[x + n]
        // Passed 0ms 2.1mb
        // let n = n as usize;
        // let mut res = vec![0; 2 * n];
        // (0..(2 * n)).for_each(|i| {
        //     let x = i / 2;
        //     res[i] = if i & 1 == 0 { nums[x] } else { nums[x + n] };
        // });
        // res


        // 方法3
        // 通过点n进行分离nums成为nums0,nums1，
        // 然后zip两个数组，
        // 再map成vec![nums0[i],nums1[i]]
        // 再flatten输出
        // nums[..n as usize].iter().zip(&nums[n as usize..]).map(|(&x, &y)| vec![x, y]).flatten().collect()

        // 方法4
        // 这个解法是题解中看到的，觉得很有启发性（下面的代码是理解了他的思想之后完成的，和题解的代码不同）
        // 因为条件限制了输入的数据在1-1000的范围，所以nums[i]的数据占用的bit只有10位，2.pow(10) = 1024
        // 那么我们可以就可以借用剩下的22位，要存入这22位，即是数字向左移动10位
        // 那么只要知道当前位置的数字nums[i]结果会是什么数字，将结果的那个数字左移动10位与当前数字共同存储在这个位置
        // 最后迭代数组nums，每个nums[i]一次向右移动10，并且只保留这10位（& 1023)，即是正确的数字
        // 为什么最后保留10位，因为某几个位置数字可能使用了拼接了之后的数字再左移10位再拼接，
        // 当然这里的解决方法既可以是结果&1023，也可以是在一开始拼接的时候就直接只要10位，也即是先 & 1023
        // Passed 0ms 2mb
        let (mut nums, n) = (nums, n as usize);
        (0..2 * n).for_each(|i| nums[i] |= if i & 1 == 0 { nums[i / 2] } else { nums[(i / 2) + n] } << 10);
        (0..2 * n).for_each(|i| nums[i] = (nums[i] >> 10) & 1023);
        nums
    }
}

#[test]
fn test() {
    // assert_eq!(Solution::shuffle(vec![], 0), vec![]);
    assert_eq!(Solution::shuffle(vec![2, 5, 1, 3, 4, 7], 3), vec![2, 3, 5, 4, 1, 7]);
    assert_eq!(Solution::shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4), vec![1, 4, 2, 3, 3, 2, 4, 1]);
    assert_eq!(Solution::shuffle(vec![1, 1, 2, 2], 2), vec![1, 2, 1, 2]);
}