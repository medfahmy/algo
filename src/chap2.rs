#![allow(unused, warnings)]

use std::collections::VecDeque;

pub fn insertion_sort<T: Ord>(slice: &mut [T]) {
    for i in 2..slice.len() {
        for j in 0..i {
            if slice[i] < slice[j] {
                slice.swap(i, j);
            }
        }
    }
}

pub fn linear_search<T: Ord>(slice: &[T], target: T) -> Option<usize> {
    for (i, x) in slice.iter().enumerate() {
        if *x == target {
            return Some(i);
        }
    }

    None
}

pub fn bit_add<const N: usize>(a: &[usize; N], b: &[usize; N]) -> [usize; N + 1] {
    let mut result = [0; N + 1];

    for i in 0..N {
        match a[i] + b[i] {
            0 => {}
            1 => {
                if result[i] == 0 {
                    result[i] = 1;
                } else if result[i] == 1 {
                    result[i] = 0;
                    result[i + 1] = 1;
                }
            }
            2 => {
                result[i + 1] += 1;
            }
            _ => {
                unreachable!("invalid bit");
            }
        }
    }

    result
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

pub fn merge_ind<T: Ord + Clone>(slice: &mut [T], p: usize, r: usize) {
    if p + 1 < r {
        let q = (r - p) / 2 + p;
        merge_ind(slice, p, q);
        merge_ind(slice, q, r);
        merge(slice, p, q, r);
    }
}

fn merge<T: Ord + Clone>(slice: &mut [T], p: usize, q: usize, r: usize) {
    assert!(p < q, "p = {}, q = {}", p, q);
    assert!(q < r, "q = {}, r = {}", q, r);

    let mut le = &slice[p..q].to_vec();
    let mut ri = &slice[q..r].to_vec();

    let x = le.get(0);

    let mut i = 0;
    let mut j = 0;

    for k in p..r {
        match (le.get(i), ri.get(j)) {
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

fn binary_search_ind<T: Ord>(
    slice: &[T],
    target: T,
    start: usize,
    end: usize,
) -> Option<usize> {
    if start == end {
        None
    } else {
        let mi = (end - start) / 2 + start;

        match target.cmp(&slice[mi]) {
            Equal => Some(mi),
            Less => binary_search_ind(slice, target, start, mi),
            Greater => binary_search_ind(slice, target, mi + 1, end),
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
    fn bit_add_works() {
        let a = [1, 0, 1];
        assert_eq!(bit_add(&a, &a), [0, 1, 0, 1]);

        let a = [1, 0, 1, 0];
        let b = [1, 0, 1, 1];
        assert_eq!(bit_add(&a, &b), [0, 1, 0, 0, 1]);
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
}
