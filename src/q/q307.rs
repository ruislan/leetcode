// 方法1
// 最朴素的方法
// 本来以为会超时的
// Passed 1444ms 22.1mb
// struct NumArray {
//     nums: Vec<i32>,
// }
//
// impl NumArray {
//     fn new(nums: Vec<i32>) -> Self {
//         NumArray { nums }
//     }
//
//     fn update(&mut self, index: i32, val: i32) {
//         self.nums[index as usize] = val;
//     }
//
//     fn sum_range(&self, left: i32, right: i32) -> i32 {
//         let left = left as usize;
//         let right = right as usize;
//         (left..=right).map(|i| self.nums[i]).sum()
//     }
// }


// 方法2
// 线段树解法
// Passed 152ms 22.2mb
// struct NumArray {
//     data: Vec<i32>,
//     nums: Vec<i32>,
// }
// #[allow(unused)]
// impl NumArray {
//     fn new(nums: Vec<i32>) -> Self {
//         let n = nums.len();
//         let data = vec![0; n * 4];
//         let mut na = NumArray { nums, data };
//         na.build_tree(0, 0, n - 1);
//         na
//     }
//
//     fn build_tree(&mut self, node: usize, lo: usize, hi: usize) {
//         if lo == hi {
//             self.data[node] = self.nums[lo];
//         } else {
//             let mid = lo + (hi - lo) / 2;
//             let left_child = node * 2 + 1;
//             let right_child = node * 2 + 2;
//             self.build_tree(left_child, lo, mid);
//             self.build_tree(right_child, mid + 1, hi);
//             self.data[node] = self.data[left_child] + self.data[right_child];
//         }
//     }
//
//     fn query_tree(&mut self, node: usize, lo: usize, hi: usize, l: usize, r: usize) -> i32 {
//         if hi < l || lo > r {
//             0
//         } else if hi <= r && lo >= l {
//             self.data[node]
//         } else if lo == hi {
//             self.data[node]
//         } else {
//             let mid = lo + (hi - lo) / 2;
//             let left_child = node * 2 + 1;
//             let right_child = node * 2 + 2;
//             let left_val = self.query_tree(left_child, lo, mid, l, r);
//             let right_val = self.query_tree(right_child, mid + 1, hi, l, r);
//             left_val + right_val
//         }
//     }
//
//     fn update_tree(&mut self, node: usize, index: usize, lo: usize, hi: usize) {
//         if lo == hi {
//             self.data[node] = self.nums[index];
//         } else {
//             let mid = lo + (hi - lo) / 2;
//             let left_child = node * 2 + 1;
//             let right_child = node * 2 + 2;
//             if index <= mid {
//                 self.update_tree(left_child, index, lo, mid);
//             } else {
//                 self.update_tree(right_child, index, mid + 1, hi);
//             }
//             self.data[node] = self.data[left_child] + self.data[right_child];
//         }
//     }
//
//     pub fn update(&mut self, index: i32, val: i32) {
//         self.nums[index as usize] = val;
//         self.update_tree(0, index as usize, 0, self.nums.len() - 1);
//     }
//
//     pub fn sum_range(&mut self, left: i32, right: i32) -> i32 {
//         self.query_tree(0, 0, self.nums.len() - 1, left as usize, right as usize)
//     }
// }

// 方法3
// 树状数组解法
// Passed 132ms 22.2mb
struct NumArray {
    data: Vec<i32>,
    nums: Vec<i32>,
}

#[allow(unused)]
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let data = vec![0; nums.len() + 1];
        let mut ft = NumArray { nums, data };
        (0..ft.nums.len()).for_each(|i| ft.update_tree(i + 1, ft.nums[i]));
        ft
    }

    const fn lower_bit(x: usize) -> usize {
        x & ((x - 1) ^ x)
    }

    fn update_tree(&mut self, mut i: usize, val: i32) {
        while i < self.data.len() {
            self.data[i] += val;
            i += NumArray::lower_bit(i);
        }
    }

    fn query_tree(&self, mut i: usize) -> i32 {
        let mut answer = 0;
        while i > 0 {
            answer += self.data[i];
            i -= NumArray::lower_bit(i);
        }
        answer
    }

    pub fn update(&mut self, index: i32, val: i32) {
        let index = index as usize;
        self.update_tree(index + 1, val - self.nums[index]);
        self.nums[index] = val;
    }

    pub fn sum_range(&mut self, left: i32, right: i32) -> i32 {
        self.query_tree(right as usize + 1) - self.query_tree(left as usize)
    }
}