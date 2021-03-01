// 其实这题虽然是简单题，
// 但是这里包含的4种解法都具有很强的基础意义，
// 所以可以经常拿它出来回味一下

// 方法1
// 存储数组，然后直接计算i..j的和
// 由于其数据量不大，所以应该可以AC的，
// 很直白，这里就不写了

// 方法2
// 前缀和
// 我们设定一个前缀和数组为pre_sum_arr来存储，
// 我们迭代进入的数组，迭代到每个数字就求出0到当前数字索引的所有数字的和
// 当然，我们不是朴素的每次都去求0..i,0..i+1,0..i+2
// 因为我们存储了0..i，所以0..i+1其实只需要pre_sum_arr[i] + nums[i]就可以了
// 这样一来，我们求i..j之间的数据就可以使用pre_sum_arr[j + 1] - pre_sum_arr[i]就可以了
// 为什么是j + 1呢，因为我们有一个初始的和是0，作为pre_sum_arr的第一个元素
// 这样一来在nums中第j个元素的前缀和实际上就是pre_sum_arr[j + 1]，
// 而nums中的第i个元素就是pre_sum_arr[i + 1]，而我们需要计算nums[i]这个元素
// 所以pre_sum_arr[i]正好是除了nums[i]这个元素以外的前缀和
// 那么0..j+1的和 - 0..i的和正好就i..j的和
// 这里我们初始化用了O(n)，而查询只用了O(1 + 1)
// AC 8ms 8.4mb
// struct NumArray {
//     pre_sum_arr: Vec<i32>
// }
//
// impl NumArray {
//     fn new(nums: Vec<i32>) -> Self {
//         let mut pre_sum_arr = vec![0];
//         let mut sum = 0;
//         for i in 0..nums.len() {
//             sum += nums[i];
//             pre_sum_arr.push(sum);
//         }
//         NumArray { pre_sum_arr }
//     }
//
//     fn sum_range(&self, i: i32, j: i32) -> i32 {
//         let i = i as usize;
//         let j = j as usize;
//         self.pre_sum_arr[j + 1] - self.pre_sum_arr[i]
//     }
// }

// 方法3
// fenwick tree
// 也叫树状数组
// 它将数字以一种树的形式进行存储，它的好处就是，将数据的更新和查询都变成了O(logn)
// 虽然查询变慢了，但是更新却变快了。它的存储方式如下，例如：
// index:       0    1    2     3    4    5    6    7    8
// data :      [1    2     3    4    5    6    7    8    9]
//              1          3         5         7         9
//             [----3]               [----11]
//             [----------------10]
//             [------------------------------------36]
// 可以看出来，奇数位置的数据都不变，
// 而偶数位置i的数据则是用了一种规则来进行计算出它的位置
// 而这个规则就是只需要当前index的二进制的最右边的那个1的位置所代表的的数值
// 例如： 10的二进制b1010，而最右边的那个1就是b10，也就是2，
// 而如何取得它呢，有两个方式x & -x，或者 x & (!x + 1)
// AC 8ms 8.3mb
// struct NumArray {
//     data: Vec<i32>,
// }
//
// #[allow(unused)]
// impl NumArray {
//     fn new(nums: Vec<i32>) -> Self {
//         let n = nums.len();
//         let mut na = NumArray { data: vec![0; n + 1] };
//         for i in 0..n {
//             na.update_tree(i + 1, nums[i]);
//         }
//         na
//     }
//
//     fn lower_bit(&self, i: usize) -> usize {
//         i & (!i + 1)
//     }
//
//     fn update_tree(&mut self, mut i: usize, x: i32) {
//         let n = self.data.len();
//         while i < n {
//             self.data[i] += x;
//             i += self.lower_bit(i);
//         }
//     }
//
//     fn query_tree(&self, mut i: usize) -> i32 {
//         let mut sum = 0;
//         while i > 0 {
//             sum += self.data[i];
//             i -= self.lower_bit(i);
//         }
//         sum
//     }
//
//     fn sum_range(&self, i: i32, j: i32) -> i32 {
//         let i = i as usize;
//         let j = j as usize;
//         self.query_tree(j + 1) - self.query_tree(i)
//     }
// }

// 方法4
// segment tree
// 又叫线段树
// AC 12ms 8.5mb
pub struct NumArray {
    pub arr: Vec<i32>,
    pub nodes: Vec<i32>,
}

#[allow(unused)]
impl NumArray {
    pub fn new(arr: Vec<i32>) -> Self {
        let n = arr.len();
        let mut na = NumArray { arr, nodes: vec![0; n << 2] };
        na.build_tree(0, 0, n - 1);
        na
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        let i = i as usize;
        let j = j as usize;
        self.query_tree(0, 0, self.arr.len() - 1, i, j)
    }

    fn query_tree(&self, node: usize, lo: usize, hi: usize, l: usize, r: usize) -> i32 {
        if r < lo || hi < l {
            0
        } else if lo >= l && hi <= r {
            self.nodes[node]
        } else if lo == hi {
            self.nodes[node]
        } else {
            let mid = lo + ((hi - lo) >> 1);
            let left_child = (node << 1) + 1;
            let right_child = (node << 1) + 2;
            let left_val = self.query_tree(left_child, lo, mid, l, r);
            let right_val = self.query_tree(right_child, mid + 1, hi, l, r);
            left_val + right_val
        }
    }

    fn build_tree(&mut self, node: usize, lo: usize, hi: usize) {
        if lo == hi {
            self.nodes[node] = self.arr[lo];
        } else {
            let mid = lo + ((hi - lo) >> 1);
            let left_child = (node << 1) + 1;
            let right_child = (node << 1) + 2;
            self.build_tree(left_child, lo, mid);
            self.build_tree(right_child, mid + 1, hi);
            self.nodes[node] = self.nodes[left_child] + self.nodes[right_child];
        }
    }
}

