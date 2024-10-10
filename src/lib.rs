pub mod bubble_sort;
pub mod bubble_sort_bad;

#[cfg(test)]
mod tests {
    use crate::{bubble_sort::bubble_sort, bubble_sort_bad::bubble_sort_bad};

    #[test]
    fn bubble_sort_test() {
        let mut input = [7, 3, 9, 12, 11];
        let sorted_result = bubble_sort(&mut input);
        assert_eq!(sorted_result, [3, 7, 9, 11, 12]);
    }

    #[test]
    fn bubble_sort_bad_test() {
        let mut input = [2, 1, 3, 4, 5];
        let sorted_result = bubble_sort_bad(&mut input);
        assert_eq!(sorted_result, [1, 2, 3, 4, 5]);
    }

}
