# 笔记

## 准备

位运算是要牢记的，数据结构和常用算法是必须熟悉熟记的。 你所使用的语言的标准库函数也是必须熟记的。 当然，刷题本身也是对这些的熟悉过程。

## 读题

时刻要注意读题，因为有些坑就在题干中，仔细读题之后能得到答案。 要注意题目中所有出现的内容的特性，

例子1，某个数字出现了两次，那么意味着这个数字相加能够被2整除，这个数字能够被异或消除等等 ，列出所有的特性，然后根据题目的要求看哪些特性是所需要的再进行选择。

例子2，数组是有序的，那么意味着可以采用二分查找或者二分的变体，那么二分查找还可以解决某个连续相等数字的上界、下界等等。

## 收藏题型

收藏题型都是我自认为非常棒，能够开阔思路，得到启发，值得无数次回味的经典题。

### 数组

#### 连续数字

连续数字，例如1,2,3,4,5..这种，或者1,3,5,7..这种连续的数字，

* 要记住等差数列求和公式Sn = n * (A1 + An) / 2，这个n可以是 An - A1 + 1。
* 要注意是有序的，有序就意味着可以二分查找
* 要注意滑动窗口，滑动窗口意味着可以两端左右移动，直到左边等于右边
* 要注意前序和

#### 找唯一

这类型的题，要注意位操作，记住异或是可以清除掉两个相同数字的，同时要注意如果在某一个粒度 上解决不了问题，可以到更细或者更粗的粒度上去解决，这取决于题目本身，这个思想我叫它放大思想，
如果在这个粒度上解决不了问题，就把这个粒度放大到更细的粒度去思考，相反，也有缩小粒度到更粗 的粒度去思考。

例如，数字就可以看做二进制组成的，可以把二进制作为存储bool值的数组。

* q31， 数组的交换技巧
* q135， 数组中值的拐点是要注意的地方，参数的记录选择也很重要，是记录方向呢？还是记录数量呢？还是记录拐点值呢？
* q239， 数组、双端队列
* q283， 数组的原地移动

### 栈

栈的先入先出的基本思想就不说了，还有一个重要的需要掌握的就是单调栈的技巧。

* q150， 逆波兰表达式，经典的栈应用了
* q224/q227， 运算表达式的解决一直都是栈的经典应用之一
* q331， 树与栈总是紧密相连，当然有时候可以用一个简单的数字来替代栈的功能

### 位操作

#### 异或类

位操作多数都是考察异或的各种技巧，所以这个只能多做题来了解了。 XOR 有很多有用的特性：

````
    x xor x=0
    0 xor x=x
    2x xor (2x+1)=1
````

* q338， 位操作没啥说的，多练多看就熟悉了。
* offer56_ii，这是一道经典的位操作应用。

### 前缀和

* q303，一个题可以学习到前缀和、树状数组、线段树的技巧，还是个简单题，真是值

### 二分查找

* q34，二分查找找上下界，经典中的经典。
* q220， 数组、二分查找、二叉平衡树。
* offer53_i，二分查找扩展应用。
* interview0803， 二分查找扩展应用。

### 滑动窗口

滑动窗口就是窗口的移动，包括扩展、收缩和整体移动。在这个窗口中可以叠加很多其他的考点进来， 例如排序、Hash、栈、堆、多个堆、队列、位操作等等，而且窗口也有固定大小的，也有视情况变大变小的，
还有的题目是将窗口题目伪装成了其他题目的样子，需要你换个角度去看才能看出原来是窗口问题。 总之就是在窗口里面办事情，关注进窗口和出窗口，选好窗口中的数据结构。其他的交给模板。 滑动窗口标准模板如下：

```rust
fn solution(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let k = k as usize;
    let mut lo = 0;
    let mut answer = 0;
    let mut window = 选好window的数据结构;
    for hi in 0..n {
        右边元素进入窗口;
        while 达到收缩窗口条件 {
            左边窗口收缩;
            lo += 1;
        }
        answer = answer.max(hi + 1 - lo);
    }
    answer as i32
}
```

* q209， 经典滑动窗口
* q424， 经典滑动窗口
* q480， 经典中的经典滑动窗口，两种解法都很值得回味，其中最优解法需要极致细节
* q992， 我最爱的一道题之一，此题我得到了很多启发，加深了将问题变成另外一个问题，解决那个问题之后把那个问题当成黑盒就解决了此题
* q995， 其实这道题跟滑动窗口关系不是特别大，虽然是固定k长度的窗口，但是这题值得回味，脑子可以很好得到锻炼
* q1052，看懂题就简单了，题干具有一定的迷惑性
* q1423，有时候有些问题看起来不简单，但是换个角度去看就变简单了，它的回味点就在于看问题的角度

### 链表

* q19， Rust如何使用raw指针和删除链表的指定节点
* q147/q148， 要用Rust玩链表之前，可以用来热身
* q234， 回文和链表的翻转，以及Rust的所有权机制

### 排序

* q164， 深刻理解桶排序
* q493， 归并排序的应用

### 回溯

我自己的感觉回溯就是一个优雅的暴力解法，将每个可能的路径都走一遍，不行了就退到上一步，然后继续走。 所以这里能做文章的地方就在于选择，题目可能给在选择的地方给你限制，比如列表不重复、列表重复、相同数字只能用一次之类的。 回溯的标准模板如下：

```rust

fn backtrace(path: &mut Vec<i32>, list: &Vec<i32>, used: &mut HashSet<i32>, answer: &mut Vec<Vec<i32>>) {
    if 条件达到 {
        加入结果集answer
        返回
    }
    遍历可供选择的列表list（排除已经选择过的used）
    for x in list {
        if used.insert(x) { // 加入已经选择
            arr.push(x);  // 加入选择
            backtrace(arr, nums, used, answer); // 继续处理
            arr.pop(); // 弹出刚刚的选择，这就是back了
            used.remove(&x); // 弹出已经选择
        }
    }
}

```

* q22， 回溯的典型应用，也算是排列组合，不过每次的选择列表需要去思考
* q46， 回溯的典型应用，全排列，这个系列都值得做一下，各种角度
* q77， 回溯的典型应用，组合，这个系列都值得做一下，各种角度
* q39/q40/q216/q377， 回溯之组合全家桶，练习回溯的不二之选
* q131， 脑子要会转弯
* q842， 回溯的典型应用

### 动态规划

* offer42，这道题算是一道动态规划题，也算是一道总结规律的题，非常具有启发性。
* q64/q120， 动态规划，最短路径
* q123， 购买股票的最佳时机系列，这个系列全都是动态规划，可以一次性复习。
* q132， 回文的dp解决方法，值得回味。
* q300/q354， 经典中的经典动态规划，三种解法意犹未尽，其中q354只是比q300多一个维度，但是两题的解法框架基本相同。
* q416， 动态规划，典型的背包问题
* q514， 动态规划的更多应用
* q714， 动态规划入门，掌握更多动态规划形态
* q1143， 教科书级别的动态规划应用，最长子公共串，这个问题可以用来解决DNA的相似度。

### 贪心算法

* q316， 字符串加贪心的应用
* q621， 贪心算法的应用
* q659， 贪心算法的应用
* q649， 贪心算法的应用
* q738， 本来很普通的一道贪心算法的应用，但是有一个大神的思路值得回味

### 并查集

* q684， 并查集的经典应用，成环检测
* q803， 并查集的高级应用，极致细节，思路爆发
* q947， 并查集的经典应用，并入集合
* q959， 并查集的应用，如何划分区域的思考
* q1489，并查集，最小生成树，思路启发
* q1631，并查集，最小生成树的变化应用，思路爆发
* q1697，并查集，很经典的一个并查集应用

### 其他

* offer62，这是一道约瑟夫环的算法解决问题。有兴趣可以模拟每次计算进行公式推导。
* q395， 我觉得还算典型的一个分治
* q448， 不使用额外空间解决问题
* q665， 极其容易错的一道easy题，很锻炼整体思维
* q765， 这个题算是出题失败的hard，导致其变成了easy。
* q1178，这个题的关键就是怎么判断二进制下的子集，这一招学会，终生受用。

## 其他总结

### 字符串

#### 日期

遇到日期类的题目，首先想到的就是闰年，即是能被4整除但不能被100整除，或者是能被400整除的。 闰年是366天，在2月份要多1天。

### 树

拿到树的题自然先想到的有几个，

* 树是特殊的图，那么广度优先和深度优先就能使用，广度优先用queue，深度优先用stack
* 树的前中后序遍历，用递归很简单，非递归都用stack，后序的非递归遍历稍微难一点，但是
    * 前序： 中 左 右
    * 中序： 左 中 右
    * 后序： 左 右 中

```rust
  fn pre_order(node: Node) {
    process(node);
    pre_order(node.left);
    pre_order(node.right);
}

fn pre_order_iter(node: Node) {
    let mut stack = vec![node];
    while !stack.is_empty() {
        node = stack.pop();
        if node != None {
            process(node);
            stack.push(node.right);
            stack.push(node.left);
        }
    }
}
```

```rust
  fn in_order(node: Node) {
    in_order(node.left);
    process(node);
    in_order(node.right);
}

fn in_order_iter(node: Node) {
    let mut stack = vec![node];
    while !stack.is_empty() {
        node = stack.pop();
        if node != None {
            if node.left != None {
                stack.push(node);
                stack.push(node.left);
            } else {
                process(node);
                stack.push(node.right);
            }
        }
    }
}
```

```rust
  fn post_order(node: Node) {
    post_order(node.left);
    post_order(node.right);
    process(node);
}
```

注意：很多Rust的初学者可能都知道clone是一个昂贵的操作，它会拷贝所有的内容，但是Rc不一样， 它只会新增一个指针和引用计数，并不昂贵，所以我刚开始做树的题的时候也觉得clone是不是拷贝了
一棵树，但是想不出更好的办法，想着先通过题吧，但是发现速度正常，内存正常啊，按道理clone会有很昂贵的 开销，然后看了一下Rc的源码，发现clone的注释就是新增一个pointer，然后引用计数增加1，哦，知识又增加了，
然后又搜了一下GitHub，发现有人在吐槽Rc的clone方法不应该叫clone方法，要不叫ref或者叫new_ref也行，嗯，我懂他。


