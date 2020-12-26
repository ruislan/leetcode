// 冒泡排序 O(n^2)
#[allow(unused)]
pub fn bubble(arr: &mut Vec<i32>) {
    for i in 0..arr.len() {
        for j in (i + 1)..arr.len() {
            if arr[i] > arr[j] {
                arr.swap(i, j);
            }
        }
    }
}

// 选择排序 O(n^2)
#[allow(unused)]
pub fn selection(arr: &mut Vec<i32>) {
    for i in 0..arr.len() {
        let mut min = i;
        for j in (i + 1)..arr.len() {
            if arr[j] < arr[min] { min = j; }
        }
        arr.swap(i, min);
    }
}

// 插入排序 O(n^2)
#[allow(unused)]
pub fn insertion(arr: &mut Vec<i32>) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

// 慢的插入排序 O(n^2)
#[allow(unused)]
pub fn slow_insertion(arr: &mut Vec<i32>) {
    for i in 1..arr.len() {
        for j in 0..i {
            if arr[i] < arr[j] {
                arr.swap(i, j);
            }
        }
    }
}

// 归并排序 O(nlogn)
#[allow(unused)]
pub fn merge_sort(arr: &mut Vec<i32>) {
    // 自顶向下
    fn sort(arr: &mut Vec<i32>, aux: &mut Vec<i32>, lo: usize, hi: usize) {
        if lo < hi { // 递归出口
            let mid = lo + (hi - lo) / 2;
            // 递归排序
            sort(arr, aux, lo, mid);
            sort(arr, aux, mid + 1, hi);

            // 合并左右两边已排序数组
            let mut i = lo; // 左边起点
            let mut j = mid + 1; // 右边起点

            if arr[mid] > arr[j] { // 如果左边的最大比右边的最小还要小，那么就已经归并了
                // 先复制数据到辅助数组
                for k in lo..=hi {
                    aux[k] = arr[k];
                }
                // 再归并左右已排序的数组
                for k in lo..=hi {
                    if i > mid { // 左边已经取尽，取右边
                        arr[k] = aux[j];
                        j += 1;
                    } else if j > hi { // 右边已经取尽，取左边
                        arr[k] = aux[i];
                        i += 1;
                    } else if aux[i] > aux[j] { // 左边比右边的大，取右边
                        arr[k] = aux[j];
                        j += 1;
                    } else { // 右边比左边大，取左边
                        arr[k] = aux[i];
                        i += 1;
                    }
                }
            }
        }
    }
    let n = arr.len();
    if n > 1 {
        sort(arr, &mut vec![0; n], 0, n - 1);
    }
}


//这个quicksort不是最优的，但是把快速排序的思想弄出来了 O(nlogn)
#[allow(unused)]
pub fn quick_sort(arr: Vec<i32>) -> Vec<i32> {
    if arr.len() < 2 {
        arr
    } else {
        let mut less = (1..arr.len()).filter(|&i| arr[i] < arr[0]).map(|i| arr[i]).collect::<Vec<i32>>();
        let mut more = (1..arr.len()).filter(|&i| arr[i] >= arr[0]).map(|i| arr[i]).collect::<Vec<i32>>();
        less = quick_sort(less);
        more = quick_sort(more);

        // 合并arr
        less.push(arr[0]);
        less.append(&mut more);
        less
    }
}

// 堆排序 O(nlogn)
#[allow(unused)]
pub fn heap_sort(arr: &mut Vec<i32>) {
    // 构建大顶堆
    fn build_heap(arr: &mut Vec<i32>) {
        let n = arr.len();
        let begin = (n - 1) / 2; // 从第一个非叶子节点开始调整堆
        for i in (0..=begin).rev() {
            adjust_heap(arr, i, n);
        }
    }

    // 调整堆
    fn adjust_heap(arr: &mut Vec<i32>, father: usize, n: usize) {
        let mut child = father * 2 + 1; // 第i个节点的左子是2i+1,右是2i+2
        while child < n { // 当前节点一直向下调整到叶子节点
            if child + 1 < n && arr[child] < arr[child + 1] {  // 找出左右节点中最大的那个
                child += 1;
            }

            if arr[child] <= arr[father] { // 父节点比子节点大，调整完成
                break;
            }

            arr.swap(child, father); // 否则，交换父子节点
            child = child * 2 + 1; // 将子节点作为父节点继续
        }
    }

    // 排序
    // 将最大的那个节点也就是arr[0]与最后一个节点交换
    // 这里最后一个节点，是指的需要排序的那个范围的最后一个节点
    // 因为需要排序的数组会不断减少（后面都是有序的）
    fn sort(arr: &mut Vec<i32>) {
        for i in (0..arr.len()).rev() {
            arr.swap(0, i); // 将堆顶的数据和最后一个换
            adjust_heap(arr, 0, i); // 这里father始终为0是因为除了0，其他的都符合堆的规则，所以只用调整0
        }
    }

    if arr.len() > 1 {
        build_heap(arr); // 先构建堆
        sort(arr); // 然后排序
    }
}


#[test]
fn test() {
    let arrays = vec![
        (vec![], vec![]), (vec![1], vec![1]), (vec![-1], vec![-1]),
        (vec![4, 3, 2, 1], vec![1, 2, 3, 4]),
        (vec![1, 1, 1, 1, 1, 0], vec![0, 1, 1, 1, 1, 1]),
        (vec![3, 5, 7, -1, 2], vec![-1, 2, 3, 5, 7]),
        (vec![3, 5, 7, 6, 4], vec![3, 4, 5, 6, 7]),
        (vec![7, 5, 3, -1, 2, 2], vec![-1, 2, 2, 3, 5, 7]),
    ];
    for mut x in arrays.clone().into_iter() {
        bubble(&mut x.0);
        assert_eq!(x.0, x.1);
    }

    for mut x in arrays.clone().into_iter() {
        selection(&mut x.0);
        assert_eq!(x.0, x.1);
    }

    for mut x in arrays.clone().into_iter() {
        insertion(&mut x.0);
        assert_eq!(x.0, x.1);
    }

    for mut x in arrays.clone().into_iter() {
        slow_insertion(&mut x.0);
        assert_eq!(x.0, x.1);
    }

    for mut x in arrays.clone().into_iter() {
        merge_sort(&mut x.0);
        assert_eq!(x.0, x.1);
    }

    for x in arrays.clone().into_iter() {
        assert_eq!(quick_sort(x.0), x.1);
    }

    for mut x in arrays.clone().into_iter() {
        heap_sort(&mut x.0);
        assert_eq!(x.0, x.1);
    }
}