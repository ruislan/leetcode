use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        // 方法1
        // 像这样的题目我们要考虑的都是拐点
        // 也就是升值变降值，降值变升值的时候
        // 假设是降值变升值：
        //     我们其实是可以不用管的，例如 3 2 1 2 3
        //     我们可以直接切断 3 2 1，然后就从2开始，因为后面都是变大就没有关系
        // 假设是升值变降值：
        //     那么我们就要考虑是不是要处理升值的最后一个，因为有可能他不够高，
        //     例如 1 2 3(4) 3 2 1，这个时候第三个就应该是4，我们就要提升最后一个的高度
        //     所以，这个时候我们就要用一个参数hi来记录最后一个升值的高度，如果降值的数量大于等于这个高度，
        //     那么就应该将这个hi的高度提升1（也就是再多发一个糖果）
        // 然后就是一种两个相等的情况了，这样的情况就是重新初始化所有状态就行了。
        // 解决这个关键点，我们就可以来思考程序了，
        // 我们用几个值来记录状态，首先up来记录升值的数量,down来记录降值的数量，answer来记录总共应该发多少糖果
        // 以及hi来记录最后一个升值的高度（他应该获得的糖果数）
        // 如果是连续升：
        //    那么将降值数量down清零，然后升值数量up+1，然后增加up个糖果到answer，最高的值设为up
        // 如果是连续降：
        //    那么将升值数量up清零，然后降值数量down+1，然后增加down个糖果到answer，
        //    这个时候如果down>=hi了，就多发一个糖果给之前的最高值那个小孩，也就是answer += 1
        // 如果相等：
        //    那么设置为初始状态，也就是up = 1,hi = up,down = 0,当然answer += 1，因为这个也要算进去嘛
        // 最后返回answer
        // Passed 0ms 2mb
        // if ratings.is_empty() { return 0; }
        //
        // let mut up = 1;
        // let mut down = 0;
        // let mut hi = up;
        // let mut answer = 1;
        // for i in 1..ratings.len() {
        //     if ratings[i - 1] == ratings[i] {
        //         answer += 1;
        //         up = 1;
        //         down = 0;
        //         hi = up;
        //     } else if ratings[i - 1] < ratings[i] {
        //         up += 1;
        //         hi = up;
        //         answer += up;
        //         down = 0;
        //     } else {
        //         down += 1;
        //         if down >= hi { answer += 1; }
        //         answer += down;
        //         up = 1;
        //     }
        // }
        // answer

        // 方法1.1
        // 我一般有重构的小习惯，所以简化了一下方法1的结构
        // Passed 0ms 2mb
        if ratings.is_empty() { return 0; }

        let mut up = 1;
        let mut down = 0;
        let mut hi = up;
        let mut answer = 1;
        for i in 1..ratings.len() {
            if ratings[i - 1] <= ratings[i] {
                up = if ratings[i - 1] == ratings[i] { 1 } else { up + 1 };
                hi = up;
                answer += up;
                down = 0;
            } else {
                down += 1;
                if down >= hi { answer += 1; }
                answer += down;
                up = 1;
            }
        }
        answer
    }
}

#[test]
fn test() {
    assert_eq!(Solution::candy(vec![1, 2, 2]), 4);
    assert_eq!(Solution::candy(vec![1, 0, 2]), 5);
}