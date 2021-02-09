use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn subarrays_with_k_distinct(a: Vec<i32>, k: i32) -> i32 {
        // 方法1
        // 暴力法
        // 每次增大窗口尺寸，每次检查窗口内的数据大小
        // 数据量大就必然会超时，O(n^3)
        // use std::collections::HashMap;
        //
        // fn check(freq:&HashMap<i32, i32>, k:usize) -> bool {
        //     let mut count = 0;
        //     for (_, v) in freq {
        //         if *v > 0 { count += 1; }
        //         if count > k { return false; }
        //     }
        //     count == k
        // }
        //
        // let n = a.len();
        // let k = k as usize;
        // let mut answer = 0;
        // for sz in k..n {
        //     let mut freq = HashMap::new();
        //     for i in 0..sz { *freq.entry(a[i]).or_insert(0) += 1; }
        //     if check(&freq, k) { answer += 1; }
        //
        //     for i in sz..n {
        //         *freq.entry(a[i]).or_insert(0) += 1;
        //         if let Some(item) = freq.get_mut(&a[i - sz]) {
        //             *item -= 1;
        //         }
        //         if check(&freq, k) { answer += 1; }
        //     }
        // }
        // answer

        // 方法2
        // 方法1的暴力优化
        // 其实只需要每次增加窗口，不再减少窗口
        // 遇到窗口内数据不合格了，就不需要检查后面的了
        // 这样的情况下我们不需要遍历freq，减少了一次循环，
        // 将O(n^3)减少到了O(n^2)，依然超时
        // 不过数据量不再是影响结果的限制，
        // 而窗口初始大小就是影响时间的关键因素了
        // !!!
        // 虽然提交超时了，但是我测试了N组极限数据，
        // 例如，几组数据量2万，每组有多个k从10~2万的，
        // 最长时间不过7~800ms。而且提交之后，只显示超时，并没有
        // 显示哪个用例超时了，所以我想至少也得在500ms以下了吧。
        // !!!
        // 看来我们必须降低到O(n)才可以完成这个题目了。
        // let n = a.len();
        // let k = k as usize;
        // let mut answer = 0;
        // for i in 0..n {
        //     let mut freq = vec![0; n + 1];
        //     let mut count = 0;
        //     for j in i..n {
        //         if freq[a[j] as usize] == 0 { count += 1; }
        //         freq[a[j] as usize] += 1;
        //         if count > k { break; }
        //         if count == k { answer += 1; }
        //     }
        // }
        // answer

        // 方法3
        // 我们要降低到O(n)的时间复杂度，我们确实搞定不了题目
        // 是不是要降低一下难度，这里是正好是k个，那我们可不可以
        // 先求出K以下都满足的数量most(k)，然后再求出K-1以下都满足的数量most(k-1)
        // 这样一来正好等于K的数量不就是most(k) - most(k-1)了吗？
        // 想想most(k) = most(k - 1) + count(k)，是吧？
        // OK，酷，这样most(k)正好是O(n)，算两个O(n)也是O(n)，我们降下来了！！！
        // 那么怎么算最小k个呢，我们首先扩展窗口，当窗口的不同数据大于k个时候，我们就停下来
        // 这个时候，我们收缩左边的窗口，直到窗口内部的不同数据又等于k
        // 例如范例： 1 2 1 2 3，k=2，定义hi右边位置，lo左边位置，
        // 初始位置0：也就是1，那么这个满足，也就是有1个了
        // 扩展：也就是1 2，这个也满足，但是有两个满足1|1 2，正好等于hi - lo + 1
        // 扩展：也就是1 2 1， 这个也满足，但是有3个满足，1 2 1|2 1|1，正好等于hi - lo + 1
        // 扩展，也就是1 2 1 2， 这个也满足，但是有4个满足，1 2 1 2|2 1 2|1 2|2，正好等于hi - lo + 1
        // 扩展，也就是1 2 1 2 3， 这个不满足
        //    收缩一直到满足，也就是2 3，这里有2个满足，2 3|3，正好等于hi - lo + 1
        // 所以这里最后满足的数量有1+2+3+4+2=12，也就是结果为所有满足条件的hi - lo + 1相加。
        // 而k-1个呢，也就是k=1，这里可以直接看出来，只要没有连续出现一样的都满足，我们可以知道其结果是5
        // 那么最后就是12 - 5 = 7，这个结果正好是范例的答案结果。
        // AC 8ms 2.3mb
        let n = a.len();
        let most = |k: i32| -> i32 {
            let mut freq = vec![0; n + 1]; // 用hashmap也是可以的，不过rust的默认hash算法安全性较高，所以效率一般
            let mut count = 0;
            let mut sum = 0;
            let mut lo = 0;
            for hi in 0..n {
                if freq[a[hi] as usize] == 0 { count += 1; }
                freq[a[hi] as usize] += 1;
                while count > k {
                    freq[a[lo] as usize] -= 1;
                    if freq[a[lo] as usize] == 0 { count -= 1; }
                    lo += 1;
                }
                sum += hi - lo + 1;
            }
            sum as i32
        };

        most(k) - most(k - 1)
    }
}