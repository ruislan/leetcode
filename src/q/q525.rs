use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        // 方法1
        // 1。算出每个位置的1和0的数量
        // 2。从i=1开始迭代nums，然后设置j从0到i，判断i和j覆盖的位置的1和0的数量是否相等
        //    如果相等，则得到最大数量，与结果对比，留下最大的那个
        // O(n^2)
        // TLE
        // let n = nums.len();
        // let mut freq = vec![vec![0_i32, 0_i32]; n];
        // let mut answer = 0_i32;
        // freq[0][nums[0] as usize] += 1;
        // for i in 1..n {
        //     freq[i][0] = freq[i - 1][0];
        //     freq[i][1] = freq[i - 1][1];
        //     freq[i][nums[i] as usize] += 1;
        //     if freq[i][0] == freq[i][1] {
        //         answer = answer.max(freq[i][0] * 2);
        //         continue;
        //     }
        //     for j in 0..i {
        //         let mut zeros = freq[i][0] - freq[j][0];
        //         let mut ones = freq[i][1] - freq[j][1];
        //         if zeros == ones {
        //             answer = answer.max(zeros * 2);
        //             break;
        //         }
        //     }
        // }
        // answer

        // 方法2
        // O(n^2)TLE了，那么我们试一下O(nlogn)，也就是说用二分法试试
        // 二分法的思路就是从1..n这个范围作为长度，来看这些长度是不是合适的，
        // 但是这里就有一个问题了，因为如果mid这个长度的1和0不等，
        // 那么到底是向大的方向还是向小的方向，因为这样都可能出现相等。
        // 所以，我们还要换一个思路，从O(n^2)能不能减少为O(n)
        // 也就是说我们必须找一个数据结构能够快速的查出i到j这两个范围内所有的0和1是相等的
        // 这里的选择肯定是hashmap了，那么k,v是什么？v不用说了，肯定是index，也就是位置
        // 而key呢，是存储0和1的个数集吗（freq_0,freq_1)？
        //       0           1           0          0         0        1         1
        // map: [(1,0),0] [(1,1),1]  [(2,1),2]  [(3,1),3] [(4,1),4] [(4,2),5] [(4,3),6]
        // 这样我们如何能够看出他们之间的关系？似乎没有一个位置的key是一样的
        // 那么这里要再变化一下思路，看怎么容易查出key，这时候突然想到如果1和0是一个数就好了
        // 既然只有两个数，我们可以把1和0的个数看成两个方向，如果他们相等就是平衡了，
        // 所以1就是1，而0可以是-1，这样我们的key就是他们平衡关系，
        //       0           1           0          0         0        1         1
        // map: [-1,0]     [0,1]      [-1,2]    [-2,3]     [-3,4]    [-2,5]    [-1,6]
        // 这里我们可以就可以看出，当key出现的时候，我们就可以通过两个位置的差值j - first得到那个平衡的长度
        // 上面的范例，我们可以看到再次出现-1的时候,有2和6，那么最大的肯定是6。再次出现-2是5-3=2，最大还是6
        // AC 28ms 3.2mb
        let n = nums.len();
        let mut map = std::collections::HashMap::new();
        let mut answer = 0;
        let mut balance = 0;
        for i in 0..n {
            balance += if nums[i] == 0 { -1 } else { 1 };
            if balance == 0 {
                answer = answer.max(i + 1);
                continue;
            }
            if let Some(&start) = map.get(&balance) {
                answer = answer.max(i - start);
            } else {
                map.insert(balance, i);
            }
        }
        answer as i32
    }
}