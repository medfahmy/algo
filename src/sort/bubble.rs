pub fn bubble_sort<T: Ord>(slice: &mut [T]) {
    for i in 0..slice.len() {
        for j in (i + 1..slice.len()).rev() {
            if slice[j] < slice[j - 1] {
                slice.swap(j, j - 1);
            }
        }
    }
}

pub fn bubble_sort2<T: Ord>(slice: &mut [T]) {
    for i in 0..slice.len() {
        for j in 0..(slice.len() - (i + 1)) {
            if slice[j] > slice[j + 1] {
                slice.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_sort_works() {
        let mut arr0: [u8; 0] = [];
        let mut arr1 = [1];
        let mut arr = [5, 2, 4, 6, 1, 3];

        bubble_sort(&mut arr0);
        assert_eq!(arr0, []);

        bubble_sort(&mut arr1);
        assert_eq!(arr1, [1]);

        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn bubble_sort2_works() {
        let mut arr0: [u8; 0] = [];
        let mut arr1 = [1];
        let mut arr = [5, 2, 4, 6, 1, 3];

        bubble_sort2(&mut arr0);
        assert_eq!(arr0, []);

        bubble_sort2(&mut arr1);
        assert_eq!(arr1, [1]);

        bubble_sort2(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
