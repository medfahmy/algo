#![allow(unused, warnings)]

pub fn insertion_sort<T: Ord>(slice: &mut [T]) {
    for i in 2..slice.len() {
        for j in 0..i {
            if slice[i] < slice[j] {
                slice.swap(i, j);
            }
        }
    }
}

pub fn selection_sort<T: Ord>(slice: &mut [T]) {
    for i in 0..slice.len() {
        for j in i..slice.len() {
            if slice[j] < slice[i] {
                slice.swap(i, j);
            }
        }
    }
}

pub fn merge_sort<T: Ord + Clone>(slice: &mut [T]) {
    merge_ind(slice, 0, slice.len());
}

pub fn merge_ind<T: Ord + Clone>(slice: &mut [T], low: usize, high: usize) {
    if low + 1 < high {
        let mid = (high - low) / 2 + low;
        merge_ind(slice, low, mid);
        merge_ind(slice, mid, high);
        merge(slice, low, mid, high);
    }
}

fn merge<T: Ord + Clone>(slice: &mut [T], low: usize, pivot: usize, high: usize) {
    assert!(low < pivot, "p = {}, q = {}", low, pivot);
    assert!(pivot < high, "q = {}, r = {}", pivot, high);

    let mut left = slice[low..pivot].to_vec();
    let mut right = slice[pivot..high].to_vec();

    let mut i = 0;
    let mut j = 0;

    for k in low..high {
        match (left.get(i), right.get(j)) {
            (None, None) => break,
            (Some(l), None) => {
                slice[k] = l.clone();
            }
            (None, Some(r)) => {
                slice[k] = r.clone();
            }
            (Some(l), Some(r)) => {
                if l <= r {
                    slice[k] = l.clone();
                    i += 1;
                } else {
                    slice[k] = r.clone();
                    j += 1;
                }
            }
        }
    }
}

use std::cmp::Ordering;
use Ordering::*;

pub fn binary_search<T: Ord>(slice: &[T], target: T) -> Option<usize> {
    binary_search_ind(slice, target, 0, slice.len())
}

fn binary_search_ind<T: Ord>(slice: &[T], target: T, start: usize, end: usize) -> Option<usize> {
    if start == end {
        None
    } else {
        let mid = (end - start) / 2 + start;

        match target.cmp(&slice[mid]) {
            Equal => Some(mid),
            Less => binary_search_ind(slice, target, start, mid),
            Greater => binary_search_ind(slice, target, mid + 1, end),
        }
    }
}

pub fn bubble_sort<T: Ord>(slice: &mut [T]) {
    for i in 0..slice.len() {
        for j in (i + 1..slice.len()).rev() {
            if slice[j] < slice[j - 1] {
                slice.swap(j, j - 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insertion_works() {
        let mut arr0: [u8; 0] = [];
        let mut arr1: [u8; 1] = [1];
        let mut arr: [u8; 6] = [5, 2, 4, 6, 1, 3];

        insertion_sort(&mut arr0);
        assert_eq!(arr0, []);

        insertion_sort(&mut arr1);
        assert_eq!(arr1, [1]);

        insertion_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn sel_works() {
        let mut arr0: [u8; 0] = [];
        let mut arr1: [u8; 1] = [1];
        let mut arr: [u8; 6] = [5, 2, 4, 6, 1, 3];

        selection_sort(&mut arr0);
        assert_eq!(arr0, []);

        selection_sort(&mut arr1);
        assert_eq!(arr1, [1]);

        selection_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn merge_works() {
        // let mut arr = [7, 1];
        // merge(&mut arr, 0, 1, 2);
        // assert_eq!(arr, [1, 7]);

        // merge_sort(&mut arr);
        // assert_eq!(arr, [0, 1, 2, 3, 4, 5, 6, 7]);

        let mut arr0: [i32; 0] = [];
        let mut arr1 = [1];
        let mut arr = [5, 2, 4, 6, 1, 3];

        merge_sort(&mut arr0);
        assert_eq!(arr0, []);

        merge_sort(&mut arr1);
        assert_eq!(arr1, [1]);

        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn binary_search_works() {
        let a = [];
        let b = [1];
        let c = [2, 4, 5, 7, 8, 19, 20, 24, 27, 30];

        assert_eq!(binary_search(&a, 1), None);

        assert_eq!(binary_search(&b, 1), Some(0));
        assert_eq!(binary_search(&b, 2), None);

        assert_eq!(binary_search(&c, 20), Some(6));
        assert_eq!(binary_search(&c, 18), None);
        assert_eq!(binary_search(&c, 7), Some(3));
    }

    #[test]
    fn bubble_works() {
        let mut arr0: [u8; 0] = [];
        let mut arr1: [u8; 1] = [1];
        let mut arr: [u8; 6] = [5, 2, 4, 6, 1, 3];

        bubble_sort(&mut arr0);
        assert_eq!(arr0, []);

        bubble_sort(&mut arr1);
        assert_eq!(arr1, [1]);

        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
