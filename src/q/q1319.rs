use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        // 方法1
        // 典型的并查集应用之一
        // 我们首先统计哪些线是冗余的，
        // 也就是说本身两个电脑已经在一个网络了，但是还是用线连了一次
        // 换成并查集的语言就是并入的时候发现已经是集合中了dup_cables
        // 然后再看已经连接的电脑数量computers，通过n - computers得到待连接电脑数
        // 然后比较待连接电脑数与dup_cables的大小：
        //    如果dup_cables大或等，也就是够连接，那么就返回待连接电脑数即可，
        //    如果dup_cables小，也就是不够连入所有的电脑，返回-1
        // 注意电脑编号是0开始的，所以computers最后还要+1
        fn find(mut x: usize, parents: &mut Vec<usize>) -> usize {
            if x != parents[x] {
                parents[x] = find(parents[x], parents);
            }
            parents[x]
        }

        fn union(mut x: usize, mut y: usize, parents: &mut Vec<usize>) -> bool {
            let mut root_x = find(x, parents);
            let mut root_y = find(y, parents);
            if root_x == root_y { return false; }
            parents[root_y] = root_x;
            true
        }

        let mut parents = vec![0; n as usize];
        for i in 0..n as usize { parents[i] = i; }

        let mut dup_cables = 0;
        let mut computers = 0;
        for i in 0..connections.len() {
            computers = computers.max(connections[i][0].max(connections[i][1]));
            if !union(connections[i][0] as usize, connections[i][1] as usize, &mut parents) {
                dup_cables += 1;
            }
        }
        let remain = n - computers - 1;
        if remain > dup_cables { -1 } else { remain }
    }
}