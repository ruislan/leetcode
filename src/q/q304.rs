// 方法1
// 由于不知道数据量大小，所以用朴素的O(n)加过去可能会超时
// 这里就不赘述了
// 所以我们起步至少也是要前缀和的
// 这里我们可以先从最简单的一维前缀和开始
// 也就是说把二维矩阵拉长成为一维矩阵
// 例如： [1 2]
//       [3 4]
// 就可以变成：[1 2 3 4]
// 而对应的索引就是row * cols + col
// 这样每个元素就有一个索引了
// 然后就是计算区域了，同样的，我们也可以把区域映射过来
// 例如[1,1] -> [2,2]成为了 1,1->1,2和2,1->2,2
// 再如[1,1] -> [3,3]成为了 1,1->1,3和2,1->2,3和3,1->3,3
// 这样我们通过计算每行的pre_sum[1,3] - pre_sum[1,1] + matrix[1,1]来解决
// 当然，如果我们不存储matrix的话，也可以用pre_sum[1,3] - pre_sum[1,0]来解决，
// 不过有个小问题就是[0,0]无法变成[0,-1]，所以我们检查一下如果是[0,0]->[0,col]就不用减去了
// AC 8ms 7.6mb
// 当然用树状数组和线段树都是可以的，而且好处是修改可以变成O(logn)，缺点都是一维的方式
// 当然也可以使用二维的前缀和方式来解决。
#[allow(unused)]
struct NumMatrix {
    pre_sum_arr: Vec<i64>,
    rows: usize,
    cols: usize,
}

#[allow(unused)]
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        if matrix.is_empty() || matrix[0].is_empty() {
            NumMatrix { pre_sum_arr: vec![], rows: 0, cols: 0 }
        } else {
            let rows = matrix.len();
            let cols = matrix[0].len();
            let mut pre_sum_arr = vec![0; rows * cols];
            let mut sum = 0_i64;
            for row in 0..rows {
                for col in 0..cols {
                    sum += matrix[row][col] as i64;
                    pre_sum_arr[row * cols + col] = sum;
                }
            }
            NumMatrix { pre_sum_arr, rows, cols }
        }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        if self.pre_sum_arr.is_empty() { return 0; }
        let row1 = row1 as usize;
        let row2 = row2 as usize;
        let col1 = col1 as usize;
        let col2 = col2 as usize;
        let mut sum = 0_i64;
        for row in row1..=row2 {
            let i1 = row * self.cols + col1;
            let i2 = row * self.cols + col2;
            sum += if i1 == 0 {
                self.pre_sum_arr[i2]
            } else {
                self.pre_sum_arr[i2] - self.pre_sum_arr[i1 - 1]
            };
        }
        sum as i32
    }
}