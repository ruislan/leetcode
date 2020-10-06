use crate::q::Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        // 要求原地排序，所以从nums中把数据搬移出来的操作是不被允许的
        // 方法1
        // 一种简单的方式就是统计红白蓝的个数
        // 然后直接重写数组
        // Passed 0ms 2mb
        // let mut colors = (0, 0, 0);
        // nums.iter().for_each(|x| match &x {
        //     0 => colors.0 += 1,
        //     1 => colors.1 += 1,
        //     _ => colors.2 += 1,
        // });
        // colors.1 = colors.0 + colors.1;
        // colors.2 = colors.1 + colors.2;
        // nums.iter_mut().enumerate().for_each(|(i, x)| {
        //     if i < colors.0 { *x = 0; } else if i < colors.1 { *x = 1; } else { *x = 2; }
        // });

        // 方法2
        // 而进阶要求是只需要一趟扫描就完成排序
        // 特点是只有0，1，2三种数据，
        // 双指针，le和ri，le从0开始,ri从len - 1开始，
        // 迭代数组直到ri为止（注意ri是变化的）
        // 如果nums[i]是2，则与ri交换，然后ri -= 1
        // 如果nums[i]是0，则与le交换，然后le += 1, i+=1
        // 如果是1， 则不需要交换，只需要i += 1
        // Passed 0ms 2mb
        let (mut i, mut le, mut ri) = (0, 0, nums.len() as i32 - 1);
        while i <= ri {
            match nums[i as usize] {
                2 => {
                    nums.swap(i as usize, ri as usize);
                    ri -= 1;
                }
                0 => {
                    nums.swap(i as usize, le as usize);
                    le += 1;
                    i += 1;
                }
                _ => i += 1,
            }
        }
    }
}

#[test]
fn test() {
    let mut v = vec![0];
    Solution::sort_colors(&mut v);
    assert_eq!(v, vec![0]);

    let mut v = vec![1];
    Solution::sort_colors(&mut v);
    assert_eq!(v, vec![1]);

    let mut v = vec![2, 2];
    Solution::sort_colors(&mut v);
    assert_eq!(v, vec![2, 2]);

    let mut v = vec![2, 0, 1];
    Solution::sort_colors(&mut v);
    assert_eq!(v, vec![0, 1, 2]);

    let mut v = vec![1, 0, 2];
    Solution::sort_colors(&mut v);
    assert_eq!(v, vec![0, 1, 2]);

    let mut v = vec![1, 2, 0];
    Solution::sort_colors(&mut v);
    assert_eq!(v, vec![0, 1, 2]);

    let mut v = vec![2, 0, 2, 1, 1, 0];
    Solution::sort_colors(&mut v);
    assert_eq!(v, vec![0, 0, 1, 1, 2, 2]);
}