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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_works() {
        let mut arr0: [u8; 0] = [];
        let mut arr1 = [1];
        let mut arr = [5, 2, 4, 6, 1, 3];

        merge_sort(&mut arr0);
        assert_eq!(arr0, []);

        merge_sort(&mut arr1);
        assert_eq!(arr1, [1]);

        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
