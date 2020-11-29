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
    let n = arr.len();
    if n > 2 {
        sort(arr, &mut vec![0; n], 0, n - 1);
    }
}


//这个quicksort不是最优的，但是把快速排序的思想弄出来了
#[allow(unused)]
pub fn quicksort(arr: Vec<i32>) -> Vec<i32> {
    if arr.len() < 2 {
        arr
    } else {
        let mut less = (1..arr.len()).filter(|&i| arr[i] < arr[0]).map(|i| arr[i]).collect::<Vec<i32>>();
        let mut more = (1..arr.len()).filter(|&i| arr[i] >= arr[0]).map(|i| arr[i]).collect::<Vec<i32>>();
        less = quicksort(less);
        more = quicksort(more);

        // 合并arr
        less.push(arr[0]);
        less.append(&mut more);
        less
    }
}


#[test]
fn test() {
    let arrays = vec![
        (vec![], vec![]), (vec![1], vec![1]), (vec![-1], vec![-1]),
        (vec![4, 3, 2, 1], vec![1, 2, 3, 4]),
        (vec![1, 1, 1, 1, 1, 0], vec![0, 1, 1, 1, 1, 1]),
        (vec![3, 5, 7, -1, 2], vec![-1, 2, 3, 5, 7]),
        (vec![3, 5, 7, -1, 2, 2], vec![-1, 2, 2, 3, 5, 7]),
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
        assert_eq!(quicksort(x.0), x.1);
    }
}