use crate::interview::Solution;

#[allow(unused)]
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        // 方法1
        // 我们需要了解2个东西
        // 1. 两个柱子中间有低柱的会形成积水
        // 2. 两边高柱中低的那个与底柱的高度差形成高，
        //    两边高柱的长度差 - 1（1是柱子本身的单位长度）形成宽，
        //    积水的单位是 高 * 宽
        // 然后我们的算法核心思路就是，迭代高度
        // 如果当前遇到更高柱子，且我们已经左边也有高柱子了，那么就形成洼地
        // 也就是要进行处理，处理的方式就是上面2中提到的方式
        // 这里需要注意的就是假设当前柱子的高度为height，左边高柱子为left_top
        // 中间的底住为bottom，那么处理完成之后，我们需要考虑新的底和高
        // 新的高就是left_top和height之间谁最高，
        // 如果height最高，那么就要看我们记录中是否还有bottom和left_top，有就要继续处理
        // 直到没有记录了，那么Height就是新的底和高
        // 如果left_top最高，那么height就是新的底
        // AC 0ms 2mb
        // let mut stack = Vec::new();
        // let mut answer = 0;
        // for i in 0..height.len() {
        //     if stack.is_empty() {
        //         stack.push((i, height[i]));
        //         continue;
        //     }
        //     let last = stack.last().unwrap();
        //     if last.1 > height[i] {
        //         stack.push((i, height[i]));
        //         continue;
        //     }
        //     if last.1 == height[i] || stack.len() < 2 {
        //         stack.pop();
        //         stack.push((i, height[i]));
        //         continue;
        //     }
        //     while stack.len() > 1 && height[i] > stack.last().unwrap().1 {
        //         let bottom = stack.pop().unwrap();
        //         let left_top = stack.pop().unwrap();
        //         let h = (left_top.1.min(height[i]) - bottom.1).abs();
        //         let w = i - left_top.0 - 1;
        //         answer += h * w as i32;
        //         if height[i] < left_top.1 {
        //             stack.push(left_top);
        //             stack.push((i, height[i]));
        //             break;
        //         }
        //         if stack.is_empty() || height[i] == left_top.1 {
        //             stack.push((i, height[i]));
        //             break;
        //         }
        //         stack.push(left_top);
        //     }
        // }
        // answer

        // 方法2
        // 核心公式就是 总雨水面积 = 总面积 - 柱子面积
        // 怎么求总面积呢，我们可以一个高度一个高度的求
        // 找出每个高度的左右两边的柱子之差再加1（1是柱子本身的宽度）就是宽，高度就是1
        // 怎么求柱子面积呢，直接把柱子的高度加起来就行了
        // AC 0ms 2mb
        let n = height.len();
        let mut sum = 0;
        let mut highest = 0;
        for &h in height.iter() { 
            sum += h;
            highest = highest.max(h);
        }
        
        let mut left = 0;
        let mut right = n - 1;        
        for h in 0..highest {
            while height[left] <= h { left += 1; }
            while height[right] <= h { right -= 1; }
            sum -= (right - left) as i32 + 1;
        }
        -sum
    }
}
