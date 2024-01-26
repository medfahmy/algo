#![allow(unused, warnings)]

use std::fmt::Debug;


fn parent(i: usize) -> usize {
    (i - 1) >> 1
}

fn left(i: usize) -> usize {
    (i << 1) + 1
}

fn right(i: usize) -> usize {
    (i << 1) + 2
}

pub fn max_heapify<T: Ord + Debug>(arr: &mut [T], i: usize, heapsize: usize) {
    let left = left(i);
    let right = right(i);

    let mut largest = 0;

    if left < heapsize && arr[left] > arr[i] {
        largest = left;
    } else {
        largest = i;
    }

    if right < heapsize && arr[right] > arr[largest] {
        largest = right;
    }

    if largest != i {
        arr.swap(i, largest);
        max_heapify(arr, largest, heapsize);
    }
}

pub fn build_max_heap<T: Ord + Debug>(arr: &mut [T]) {
    let n = arr.len() / 2;

    for i in (0..n).rev() {
        max_heapify(arr, i, arr.len());
    }
}

pub fn heap_sort<T: Ord + Debug>(arr: &mut [T]) {
    build_max_heap(arr);
    let mut heapsize = arr.len();

    for i in (1..arr.len()).rev() {
        arr.swap(0, i);
        heapsize -= 1;
        max_heapify(arr, 0, heapsize);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics_work() {
        assert_eq!(parent(1), 0);
        assert_eq!(parent(2), 0);

        assert_eq!(parent(3), 1);
        assert_eq!(parent(4), 1);

        assert_eq!(left(1), 3);
        assert_eq!(right(1), 4);

        assert_eq!(left(3), 7);
        assert_eq!(right(3), 8);
    }

    #[test]
    fn max_heapify_works() {
        let mut arr = [16, 4, 10, 14, 7, 9, 3, 2, 8 , 1];
        let heapsize = arr.len();
        let expected = [16, 14, 10, 8, 7, 9, 3, 2, 4 , 1];

        max_heapify(&mut arr, 1, heapsize);

        assert_eq!(arr, expected);
    }

    #[test]
    fn build_max_heap_works() {
        let mut arr = [4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
        let expected = [16, 14, 10, 8, 7, 9, 3, 2, 4 , 1];

        build_max_heap(&mut arr);

        assert_eq!(arr, expected);
    }

    #[test]
    fn heapsort_works() {
        let mut arr0: [u8; 0] = [];
        let mut arr1: [u8; 1] = [1];
        let mut arr: [u8; 6] = [5, 2, 4, 6, 1, 3];

        heap_sort(&mut arr0);
        assert_eq!(arr0, []);

        heap_sort(&mut arr1);
        assert_eq!(arr1, [1]);

        heap_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
