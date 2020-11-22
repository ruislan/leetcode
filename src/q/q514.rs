use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        // 方法1
        // 迭代取出key[i]
        // 我们要在环中找到key[i]，那么只有两种方式，向左或者向右
        // 这样我们就构成了一个二叉树，那么问题就变成了二叉树求最小路径和
        // 例如：“godding”和"gd"
        // 首先让待处理的位置为pos = 0，也就是ring[0]
        // 那么由于ring[0] == "g"，所以我们把不走和向左都认为是左子树 0 + 1
        // 那向右走需要多少呢，我们从0开始线性搜索得到ring[6] == "g"，也就是右子树是6 + 1
        //   0
        // 1  7
        // 再来看“d”,我们现在的位置取左子树的位置是pos = 0，向左走到头再从尾部再左，4步也就是len - 4 = 3，ring[3] = "d"，左子树是4 + 1 = 5
        // 向右，2步，也就是ring[2] = "d"，右子树是 2 + 1 = 3
        //        0
        //     1     7
        //   5   3
        // 再来看右子树，右子树的位置pos = 6，向左，3步可到，位置变成了3
        // 向右，到底部，再从头部开始，3步可到，位置变成了2
        //        0
        //     1     7
        //   5   3 4   4
        // 那么，最后可以看到，我们的树里面每条根到子叶节点的求和得到6 4 11 11，取最小值4即是答案
        // 那么，如果需要查找的数字太长，那么我们计算的量会成指数倍增加，所以这个解其实是会超时的
        // Not Passed 超出时间限制
        // let ring = ring.chars().into_iter().collect::<Vec<char>>();
        // let mut leafs = vec![(0, 0)];
        // for ch in key.chars() {
        //     let mut new_leafs = Vec::new();
        //     for leaf in leafs {
        //         // search left
        //         let mut pos = leaf.0;
        //         let mut val = leaf.1 + 1;
        //         while ring[pos] != ch {
        //             if pos == 0 { pos = ring.len() - 1; } else { pos -= 1; }
        //             val += 1;
        //         }
        //         new_leafs.push((pos, val));
        //
        //         // search right
        //         let mut pos = if leaf.0 == ring.len() - 1 { 0 } else { leaf.0 + 1 };
        //         let mut val = leaf.1 + 2;
        //         while ring[pos] != ch {
        //             if pos == ring.len() - 1 { pos = 0; } else { pos += 1; }
        //             val += 1;
        //         }
        //         new_leafs.push((pos, val));
        //     }
        //     leafs = new_leafs;
        // }
        // leafs.into_iter().map(|x| x.1).min().unwrap_or(0)

        // 方法2
        // 方法1的超时说明了我们要么优化它，要么换个方式
        // 接下来，我们分析一下，其实我们一开始有两种方式，
        //    第一种：就是方法1，我们记录累加每次向左和向右的数据，最后找出最小的那个
        //    第二种：我们可以先确定当前pos到达key[i]所在的位置的数据，每个都记录下来。
        //           然后基于每一个之前的位置再找做前面的操作，然后直到最后的字符，我们再找出最小的那个即可。
        // 然后，我们扩展开来说第二种，先从例子开始：（注意移动的时候要取小的那个方向）
        // 例如： gogogo， 当前我们要找 "og"
        // 起始位置肯定是0，且只有一个。
        //     处理“o”：那么第一个o在位置1，第二个在位置3，第三个在位置5
        //             现在我们有了位置[1,3,5] = 步数[2, 4, 2]
        //     处理“g”：
        //        位置1的o到所有的g的数据是：
        //            第一个g在位置0，移动的步数1，得到结果是2 + 1 + 1 = 4
        //            第二个g在位置2，移动的步数1，得到结果是2 + 1 + 1 = 4
        //            第三个g在位置4，移动的步数3，得到结果是2 + 3 + 1 = 6
        //        位置3的o到所有的g的数据是：
        //            第一个g在位置0，移动的步数2，得到结果是4 + 2 + 1 = 7
        //            第二个g在位置2，移动的步数1，得到结果是4 + 1 + 1 = 6
        //            第三个g在位置4，移动的步数1，得到结果是4 + 1 + 1 = 6
        //        位置5的o到所有的g的数据是：
        //            第一个g在位置0，移动的步数1，得到结果是2 + 1 + 1 = 4
        //            第二个g在位置2，移动的步数3，得到结果是2 + 3 + 1 = 6
        //            第三个g在位置4，移动的步数1，得到结果是2 + 1 + 1 = 4
        // 然后，现在最后处理得到位置[0,2,4] = 步数[4 , 4 , 4]，然后我们找到最小的那个是4
        // 我们把这个方法转换成动态规划
        // 首先是dp[row][col]中的row和col分别是什么:
        //    从上面可知：row就是key[i]
        //              col就是ring的每个位置
        // 然后是dp[row][col]存储的是什么：
        //    从上面可知：是累加的步数
        // 下面是dp[0][col]初始化是什么：
        //    这里我们知道要找最小的，所以我们初始化一个很大的值
        //    这个值得大小，其实就是ring的长度乘以key的长度，因为最长的步骤就是ring.len() * key.len()
        // 接着是状态转移是什么：
        //    从上面可知：dp[row][col] = min(dp[row][col], dp[row - 1][每个当前key的位置k_col] + 移动的步数 + 1)
        //       这里面移动的步数我们需要知道前一个key的每一个位置在哪里（从哪里开始），
        //       所以我们需要一个辅助数组来存储ring中每个字符在ring中的各个位置，记为pos
        //       移动的步数 = min((pos[key[row - 1]][i] - k_col).abs(), ring.len() - (pos[key[row - 1]][i] - k_col).abs())
        // 最后我们得到什么：
        //    从上面可知，最后一组状态的最小值
        // Passed 4ms 2mb
        // let mut dp = vec![vec![10000_i32; ring.len()]; key.len()];
        // let key = key.into_bytes();
        //
        // // 初始化每个ring中字符的位置
        // let mut pos = vec![Vec::new(); 26];
        // for (i, ch) in ring.bytes().enumerate() {
        //     pos[(ch - b'a') as usize].push(i);
        // }
        //
        // // 初始化第一行
        // for &p in pos[(key[0] - b'a') as usize].iter() {
        //     dp[0][p] = p.min(ring.len() - p) as i32 + 1;
        // }
        //
        // let size = ring.len() as i32;
        // for row in 1..key.len() {
        //     for &col in pos[(key[row] - b'a') as usize].iter() {
        //         for &last_col in pos[(key[row - 1] - b'a') as usize].iter() {
        //             let c = col as i32;
        //             let p = last_col as i32;
        //             dp[row][col] = dp[row][col].min(dp[row - 1][last_col] + (c - p).abs().min(size - (c - p).abs()) + 1);
        //         }
        //     }
        // }
        // *dp.last().unwrap().into_iter().min().unwrap()

        // 方法3
        // 方法2我们用了ring.len() * key.len()的空间，实际上我们可以只要记录每次的状态即可
        // 也就是只要ring.len()的空间即可
        // 然后我们取最后那个字符的位置中的最小值进行返回就行了
        // Passed 0ms 2mb
        let mut dp = vec![0_i32; ring.len()];
        let key = key.into_bytes();

        let mut pos = vec![Vec::new(); 26];
        for (i, ch) in ring.bytes().enumerate() {
            pos[(ch - b'a') as usize].push(i);
        }

        for &p in pos[(key[0] - b'a') as usize].iter() {
            dp[p] = p.min(ring.len() - p) as i32 + 1;
        }
        let size = ring.len() as i32;
        for row in 1..key.len() {
            for &col in pos[(key[row] - b'a') as usize].iter() {
                dp[col] = pos[(key[row - 1] - b'a') as usize].iter().map(|&last_col| {
                    let c = col as i32;
                    let p = last_col as i32;
                    dp[last_col] + (c - p).abs().min(size - (c - p).abs()) + 1
                }).min().unwrap();
            }
        }
        pos[(key[key.len() - 1] - b'a') as usize].iter().map(|i| dp[*i]).min().unwrap()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_rotate_steps(String::from("g"), String::from("g")), 1);
    assert_eq!(Solution::find_rotate_steps(String::from("godding"), String::from("gd")), 4);
    assert_eq!(Solution::find_rotate_steps(String::from("godding"), String::from("god")), 5);
    assert_eq!(Solution::find_rotate_steps(String::from("xrrakuulnczywjs"), String::from("jrlucwzakzussrlckyjjsuwkuarnaluxnyzcnrxxwruyr")), 204);
    assert_eq!(Solution::find_rotate_steps(String::from("caotmcaataijjxi"), String::from("oatjiioicitatajtijciocjcaaxaaatmctxamacaamjjx")), 137);
}