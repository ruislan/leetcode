use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        // 方法1
        // 通过观察数组我们可以看到实际上是一个带权的有向图查询问题
        // 例如[['a','b'],['b','c']]，values = [2.0,3.0], queries = [['a','c'],['b','a'],['a','a'], ['x','x']
        // 这里查询a -> c，实际上就是要找到a和c的关系，那么知道a / b = 2 , b / c = 3
        // 得到 a/c = a/b * b/c 也就是说找到a -> b -> c，将这两个边的权乘起来就行了
        // 那么我们就按照这个思路来写步骤
        // 首先是构成图，这里我们可以利用rust的元组直接将权重和到达的节点绑定，例如：
        // [a, [(b,2)]], [b, [(a,1/2),(c,3)], [c, [(b,1/3)]]
        // 然后是迭代查询，利用广度或者深度优先搜索查找下一个节点
        // 这里需要注意的地方就是，
        //    有可能查询的起点和终点有一个图上，可以直接处理
        //    还有可能是查询的起点和终点在图上，但是是同一个，也可以直接处理
        //    深度或者广度搜索都检索完了，但是没有找到
        // 然后就是正常的深度和广度搜索，
        //    利用hashset存储已经访问过的值，以免引起循环检索
        //    利用Rust的元组可以让每次检索的中间值与检索到的节点绑定
        // 最后返回结果
        // Passed 0ms 1.9mb
        let n = equations.len();
        let mut graph = std::collections::HashMap::new();

        for i in 0..n {
            let eq = &equations[i];
            let val = values[i];
            graph.entry(eq[0].clone()).or_insert(Vec::new()).push((eq[1].clone(), val));
            graph.entry(eq[1].clone()).or_insert(Vec::new()).push((eq[0].clone(), 1_f64 / val));
        }

        let mut answer = Vec::new();
        for query in queries {
            if !graph.contains_key(&query[0]) || !graph.contains_key(&query[1]) {
                answer.push(-1_f64);
            } else if query[0] == query[1] {
                answer.push(1_f64);
            } else {
                let mut pasts = std::collections::HashSet::new();
                pasts.insert(query[0].clone());
                let mut val = -1_f64;
                let mut stack = vec![(query[0].clone(), 1_f64)];
                while !stack.is_empty() {
                    let node = stack.pop().unwrap();
                    let next_nodes = graph.get(&node.0).unwrap();
                    for next_node in next_nodes {
                        if pasts.insert(next_node.0.clone()) {
                            stack.push((next_node.0.clone(), node.1 * next_node.1));
                            if next_node.0 == query[1] {
                                val = node.1 * next_node.1;
                                stack.clear();
                                break;
                            }
                        }
                    }
                }
                answer.push(val);
            }
        }
        answer
    }
}