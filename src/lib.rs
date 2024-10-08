mod bubble_sort;

#[cfg(test)]
mod tests {
    use crate::bubble_sort::bubble_sort;

    #[test]
    fn bubble_sort_test() {
        let mut input = [7, 3, 9, 12, 11];
        let sorted_result = bubble_sort(&mut input);
        assert_eq!(sorted_result, [3, 7, 9, 11, 12]);
    }
}
