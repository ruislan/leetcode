use crate::lcp::Solution;

impl Solution {
    pub fn num_ways(n: i32, relation: Vec<Vec<i32>>, k: i32) -> i32 {
        // 方法1
        // 创建长度为n的数组arr，arr[i]存储了每个节点到其他节点的列表arr[i] : Vec<i32>
        // 迭代arr[0]（起点的第一个节点，每个节点又迭代下一个节点，下一个节点又迭代下一个节点，直到达到k，非K深度非终结点）
        // 达到k的情况下，统计节点是4的数量即可
        // 例如n = 5, relation = [[0,2],[2,1],[3,4],[2,3],[1,4],[2,0],[0,4]], k = 3
        // 得到arr[0] = [2,4],arr[1] = [4], arr[2] = [1,3,0], arr[3] = [4]
        // 迭代arr[0]，k = 1， [2,4]，取2
        //     arr[2]，k = 2， 取 1，3，0迭代
        //         arr[1], k = 3， [4]，取4，count += 1
        //         arr[3]， k = 3， [4]，取4，count += 1
        //         arr[0], k = 3， [2,4]，取4，count += 1
        // 迭代结束
        // 看起来就像初始节点构造了一棵树，直到k深度结束，然后统计深度为k的节点值为4的数量
        0
    }
}