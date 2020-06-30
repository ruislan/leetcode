struct Solution;

impl Solution {
    pub fn min_cost_to_move_chips(chips: Vec<i32>) -> i32 {
        // 方法1
        // 断断续续看了好几次，终于看懂了这道题的题目
        // 原来chips中存储的是位置chips[i]就表示第i个位置，而不是有几个筹码
        // 所以[2,2,2,3,3]表示位置2有3个筹码，位置3有2个筹码
        // 这样一来我们就很好处理了，因为向左右移动2个单位的位置代价为0，向左右移动1个单位的位置代价为1
        // 那么也即是说，如果在同一位置，或者都是奇数位置，或者都是偶数位置，那么不需要移动，或者代价都是0（移动2个单位代价为0）
        // 如果位置有奇数和偶数，那么我们移动奇数到偶数或者移动偶数到奇数，找出筹码最小的那个组
        // 所以我们列两堆，一堆表示奇数odd,一堆表示偶数even
        // 然后迭代chips，位置是奇数的odd += 1，位置是偶数的even += 1
        // 然后取odd和even的最小值就行，全odd或者全even位置的，相互的对位都是0，结果为0完全正确
        // 有奇有偶的，最小值就是那个结果
        // Passed 0ms 2mb

        let (mut odd, mut even) = (0, 0);
        chips.iter().for_each(|&x| { if x & 1 != 0 { odd += 1; } else { even += 1; } });
        std::cmp::min(odd, even)

        // std::cmp::min(chips.iter().filter(|&&x| x & 1 != 0).count(), chips.iter().filter(|&&x| x & 1 == 0).count()) as i32
    }
}

#[test]
fn test() {
    // assert_eq!(Solution::min_cost_to_move_chips(vec![]), );
    assert_eq!(Solution::min_cost_to_move_chips(vec![1, 2, 3]), 1);
    assert_eq!(Solution::min_cost_to_move_chips(vec![1, 2]), 1);
    assert_eq!(Solution::min_cost_to_move_chips(vec![1, 2, 3, 4]), 2);
    assert_eq!(Solution::min_cost_to_move_chips(vec![2, 2, 4, 4]), 0);
    assert_eq!(Solution::min_cost_to_move_chips(vec![2, 2, 2, 3, 3]), 2);
    assert_eq!(Solution::min_cost_to_move_chips(vec![3, 3, 3, 3, 3]), 0);
}