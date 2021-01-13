#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SegmentTree {
    pub arr: Vec<i32>,
    pub nodes: Vec<i32>,
}

#[allow(unused)]
impl SegmentTree {
    pub fn new(arr: Vec<i32>) -> Self {
        let n = arr.len();
        let mut st = SegmentTree { arr, nodes: vec![0; n << 2] };
        st.build_tree(0, 0, n - 1);
        st
    }

    pub fn sum_of_range(&self, l: usize, r: usize) -> i32 {
        self.query_tree(0, 0, self.arr.len() - 1, l, r)
    }

    pub fn update(&mut self, i: usize, val: i32) {
        self.arr[i] = val;
        self.update_tree(0, 0, self.arr.len() - 1, i);
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

    fn update_tree(&mut self, node: usize, lo: usize, hi: usize, i: usize) {
        if lo == hi {
            self.nodes[node] = self.arr[i];
        } else {
            let mid = lo + ((hi - lo) >> 1);
            let left_child = (node << 1) + 1;
            let right_child = (node << 1) + 2;
            if i <= mid {
                self.update_tree(left_child, lo, mid, i);
            } else {
                self.update_tree(right_child, mid + 1, hi, i);
            }
            self.nodes[node] = self.nodes[left_child] + self.nodes[right_child];
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