use crate::q::Solution;

impl Solution {
    pub fn video_stitching(clips: Vec<Vec<i32>>, t: i32) -> i32 {
        // 方法1
        // 创建一个长度为t+1的数组video,初始都为0,数组存储的是这个位置的clips的个数
        // video[0]永远是0，
        // 从1位置到t位置开始迭代：
        //   然后遍历每个clips：
        //      当clips[i]的起点在当前位置之前，终点在当前位置之后，那么：
        //          video[i] = video[i].min(1 + video[x[0]])
        //          这里解释一下求最小值的用意，我们可以假设我们现在选择当前这一条，
        //          那么是不是我们的计数就多了一个，然后我们这个占用的位置是video[x[0]..=x[1]]，
        //          那么意味着是不是还要看video[x[0]]这个位置保存的最小组成个数是多少加上我们这个1就是
        //          我们选择的这个video[x[1]]的个数，那么用这个个数和video[x[1]]中本身已经存的个数去
        //          比较，看谁小，我们就取谁的值。
        // 迭代完成，查看video[t]位置的值，如果等于t+1那么就返回-1，如果不是返回video[t]
        // Passed 0ms 2mb
        // let mut video = vec![t + 1; t as usize + 1];
        // video[0] = 0;
        // (1..video.len()).for_each(|i| {
        //     for x in clips.iter() {
        //         if (x[0] as usize) < i && i <= (x[1] as usize) { video[i] = video[i].min(1 + video[x[0] as usize]); }
        //     }
        // });
        // if video[t as usize] == t + 1 { -1 } else { video[t as usize] }

        // 方法2
        // Passed 0ms 2.2mb
        let mut video = vec![0; t as usize];
        for x in clips {
            if x[0] < t { video[x[0] as usize] = video[x[0] as usize].max(x[1]); }
        }
        let mut max = 0;
        let mut prev = 0;
        let mut count = 0;
        for i in 0..t as usize {
            max = max.max(video[i]);
            if i == max as usize { return -1; }
            if i == prev as usize {
                count += 1;
                prev = max;
            }
        }
        count
    }
}

#[test]
fn test() {
    assert_eq!(Solution::video_stitching(vec![vec![0, 0]], 1), -1);
    assert_eq!(Solution::video_stitching(vec![vec![0, 1]], 1), 1);
    assert_eq!(Solution::video_stitching(vec![vec![0, 2], vec![4, 8]], 5), -1);
    assert_eq!(Solution::video_stitching(vec![vec![0, 1], vec![1, 2]], 5), -1);
    assert_eq!(Solution::video_stitching(vec![vec![0, 4], vec![2, 8]], 5), 2);
    assert_eq!(Solution::video_stitching(vec![vec![0, 4], vec![2, 5], vec![3, 6]], 5), 2);
    assert_eq!(Solution::video_stitching(vec![vec![0, 2], vec![4, 6], vec![8, 10], vec![1, 9], vec![1, 5], vec![5, 9]], 10), 3);
    assert_eq!(Solution::video_stitching(vec![vec![0, 1], vec![6, 8], vec![0, 2],
                                              vec![5, 6], vec![0, 4], vec![0, 3],
                                              vec![6, 7], vec![1, 3], vec![4, 7],
                                              vec![1, 4], vec![2, 5], vec![2, 6],
                                              vec![3, 4], vec![4, 5], vec![5, 7],
                                              vec![6, 9]], 9), 3);
    assert_eq!(Solution::video_stitching(vec![vec![2, 3]], 1), -1);
}