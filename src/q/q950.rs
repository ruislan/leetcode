use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
        // 方法1
        // 我们来看范例：
        // ---
        // 输入：[17,13,11,2,3,5,7]
        // 输出：[2,13,3,11,5,17,7]
        // 解释：
        // 我们得到的牌组顺序为 [17,13,11,2,3,5,7]（这个顺序不重要），然后将其重新排序。
        // 重新排序后，牌组以 [2,13,3,11,5,17,7] 开始，其中 2 位于牌组的顶部。
        // 我们显示 2，然后将 13 移到底部。牌组现在是 [3,11,5,17,7,13]。
        // 我们显示 3，并将 11 移到底部。牌组现在是 [5,17,7,13,11]。
        // 我们显示 5，然后将 17 移到底部。牌组现在是 [7,13,11,17]。
        // 我们显示 7，并将 13 移到底部。牌组现在是 [11,17,13]。
        // 我们显示 11，然后将 17 移到底部。牌组现在是 [13,17]。
        // 我们展示 13，然后将 17 移到底部。牌组现在是 [17]。
        // 我们显示 17。
        // ---
        // 通过范例，我们可以把每一步的数组提炼出来：
        // [17] -> [13, 17] -> [11, 17, 13] -> [7, 13, 11, 17]...
        // 也就是说，如果我只有一张牌的情况下，直接放进去就好了
        // 2张牌[13]，把第二张牌插入到头部
        // 3张牌[11]，可能是先把13放到尾部，也可能是把17放到头部，我们后面来看
        // 4张牌[7]，这里可以看出来了，是把尾部的放到头部，然后再插入新牌到头部
        // 所以，通过模拟操作可以归纳如下：
        // 1. 首先逆序排列deck
        // 2. 然后将deque的右边放到左边
        // 3. 将deck[i]放到最前面
        // AC 0ms 2.1mb
        let mut deque = std::collections::VecDeque::new();
        let mut deck = deck;
        deck.sort_unstable_by(|a, b| b.cmp(a));
        for card in deck {
            if !deque.is_empty() {
                let right = deque.pop_back().unwrap();
                deque.push_front(right);
            }
            deque.push_front(card);
        }
        deque.into_iter().collect()
    }
}