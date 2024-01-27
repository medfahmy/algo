use std::cmp::Ordering;
use Ordering::*;

pub fn binary_search<T: Ord>(slice: &[T], target: T) -> Option<usize> {
    binary_search_ind(slice, target, 0, slice.len())
}

fn binary_search_ind<T: Ord>(slice: &[T], target: T, start: usize, end: usize) -> Option<usize> {
    if start == end {
        return None;
    } 

    let mid = (end - start) / 2 + start;

    match target.cmp(&slice[mid]) {
        Equal => Some(mid),
        Less => binary_search_ind(slice, target, start, mid),
        Greater => binary_search_ind(slice, target, mid + 1, end),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
