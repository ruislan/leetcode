use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        // 方法1
        // 首先检查第一个位置，如果为0，那么放置一盆花
        // 然后检查1..flowerbed.len()：
        //    如果[i - 1]位置有花，且[i]位置也有花：
        //       那么将[i - 1]位置的花清除
        //       这里注意的是，i - 1必然是我们放置的花，所以我们可以清除
        //    如果[i - 1]位置没有花，且[i]位置也没有花：
        //       那么将[i]位置放置花
        // 返回放置的数量是否大于等于n
        // Passed 0ms 2.1mb
        let mut flowerbed = flowerbed;
        let mut count = 0;
        if flowerbed[0] == 0 {
            count += 1;
            flowerbed[0] = 1;
        }
        for i in 1..flowerbed.len() {
            if flowerbed[i - 1] == 1 && flowerbed[i] == 1 {
                count -= 1;
                flowerbed[i - 1] = 0;
            }
            if flowerbed[i - 1] == 0 && flowerbed[i] == 0 {
                count += 1;
                flowerbed[i] = 1;
            }
        }
        count >= n
    }
}