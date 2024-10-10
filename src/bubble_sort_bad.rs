// swapped boolean is not used, which makes the performance bad
pub fn bubble_sort_bad(input: &mut [usize]) -> &mut [usize] {
    let n = input.len();
    for i in 0..n - 1 {
        for j in 0..n - i - 1 {
            if input[j] > input[j + 1] {
                let smaller = input[j + 1];
                let greater = input[j];
                input[j] = smaller;
                input[j + 1] = greater;
            }
        }
    }
    input
}
