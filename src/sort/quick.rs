use std::fmt::Debug;

pub fn quicksort<T: Ord + Debug>(arr: &mut [T]) {
    let len = arr.len();
    quicksort_rec(arr, 0, len);
}

fn quicksort_rec<T: Ord + Debug>(arr: &mut [T], low: usize, high: usize) {
    if low + 1 >= high {
        return;
    }

    let pivot = partition(arr, low, high - 1);

    quicksort_rec(arr, low, pivot);
    quicksort_rec(arr, pivot + 1, high);
}

fn partition<T: Ord + Debug>(arr: &mut [T], low: usize, high: usize) -> usize {
    let mut pivot = low;

    for i in low..high {
        if arr[i] < arr[high] {
            arr.swap(pivot, i);
            pivot += 1;
        }
    }

    arr.swap(pivot, high);
    
    pivot
}

// fn partition2<T: Ord + Debug>(arr: &mut [T], low: usize, high: usize) -> usize {
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partition_works() {
        let mut arr = [5, 2, 4, 6, 1, 3];

        let len = arr.len();
        let pivot = partition(&mut arr, 0, len - 1);

        assert_eq!(&arr[pivot], &3);
        assert_eq!(&arr[..pivot], [2, 1]);
        assert_eq!(&arr[pivot + 1..], [6, 5, 4]);
    }

    #[test]
    fn quicksort_works() {
        let mut arr0: [u8; 0] = [];
        let mut arr1 = [1];
        let mut arr = [5, 2, 4, 6, 1, 3];

        quicksort(&mut arr0);
        assert_eq!(arr0, []);

        quicksort(&mut arr1);
        assert_eq!(arr1, [1]);

        quicksort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
