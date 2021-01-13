#[derive(PartialEq, Eq, Clone, Debug)]
pub struct FenwickTree {
    pub arr: Vec<i32>,
    pub nodes: Vec<i32>,
}

#[allow(unused)]
impl FenwickTree {
    pub fn new(arr: Vec<i32>) -> Self {
        let data = vec![0; arr.len() + 1];
        let mut ft = FenwickTree { arr, nodes: data };
        (0..ft.arr.len()).for_each(|i| ft.update_tree(i + 1, ft.arr[i]));
        ft
    }

    const fn lower_bit(x: usize) -> usize {
        x & ((x - 1) ^ x)
    }

    fn update_tree(&mut self, mut i: usize, val: i32) {
        while i < self.nodes.len() {
            self.nodes[i] += val;
            i += FenwickTree::lower_bit(i);
        }
    }

    fn query_tree(&self, mut i: usize) -> i32 {
        let mut answer = 0;
        while i > 0 {
            answer += self.nodes[i];
            i -= FenwickTree::lower_bit(i);
        }
        answer
    }

    pub fn update(&mut self, i: usize, val: i32) {
        self.update_tree(i + 1, val - self.arr[i]);
        self.arr[i] = val;
    }

    pub fn sum_of_range(&self, begin: usize, end: usize) -> i32 {
        self.query_tree(end + 1) - self.query_tree(begin)
    }
}