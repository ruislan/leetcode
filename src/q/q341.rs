#[derive(Debug, PartialEq, Eq)]
#[allow(unused)]
pub enum NestedInteger {
  Int(i32),
  List(Vec<NestedInteger>)
}

struct NestedIterator {
    values: Vec<i32>,
    cursor: usize,
}

// 方法1
// 用递归直接将所有的数据取出来即可
// AC 0ms 3mb 44/44
#[allow(unused)]
impl NestedIterator {

    fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut values = Vec::new();
        NestedIterator::unpack(nested_list, &mut values);
        NestedIterator { values, cursor: 0 }
    }

    fn unpack(nested_list: Vec<NestedInteger>, values: &mut Vec<i32>) {
        for ni in nested_list.into_iter() {
            match ni {
                NestedInteger::Int(x) => values.push(x),
                NestedInteger::List(list) => NestedIterator::unpack(list, values)
            }
        }
    }
    
    fn next(&mut self) -> i32 {
        let next = self.values[self.cursor];
        self.cursor += 1;
        next
    }
    
    fn has_next(&self) -> bool {
        self.cursor < self.values.len()
    }
}
