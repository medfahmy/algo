pub fn selection_sort<T: Ord>(slice: &mut [T]) {
    for i in 0..slice.len() {
        for j in i..slice.len() {
            if slice[j] < slice[i] {
                slice.swap(i, j);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn selection_sort_works() {
        let mut arr0: [u8; 0] = [];
        let mut arr1 = [1];
        let mut arr = [5, 2, 4, 6, 1, 3];

        selection_sort(&mut arr0);
        assert_eq!(arr0, []);

        selection_sort(&mut arr1);
        assert_eq!(arr1, [1]);

        selection_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
}
