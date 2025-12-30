use itertools::Itertools;
use std::fmt::Display;

fn all_equal<T: PartialEq>(mut iter: impl Iterator<Item = T>) -> bool {
    iter.next()
        .map(|first| iter.all(|x| x == first))
        .unwrap_or(false)
}

fn is_arithmetic_progression<T: Copy + PartialOrd + std::ops::Sub<Output = T> + PartialEq>(
    nums: &[T],
) -> bool {
    all_equal(
        nums.iter()
            .copied()
            .sorted_by(|a, b| a.partial_cmp(b).unwrap())
            .tuple_windows()
            .map(|(a, b)| b - a),
    )
}

fn print_result<T: Display>(example: usize, nums: &[T], result: bool) {
    print!("Example {}:\nInput: [", example);
    for (i, num) in nums.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{}", num);
    }
    println!("]\nOutput: {}\n", result);
}

fn main() {
    let nums1 = vec![1, 3, 5, 7, 9];
    let nums2 = vec![9, 1, 7, 5, 3];
    let nums3 = vec![1, 2, 4, 8, 16];
    let nums4 = vec![5, -1, 3, 1, -3];
    let nums5 = vec![1.5, 3.0, 0.0, 4.5, 6.0];

    print_result(1, &nums1, is_arithmetic_progression(&nums1));
    print_result(2, &nums2, is_arithmetic_progression(&nums2));
    print_result(3, &nums3, is_arithmetic_progression(&nums3));
    print_result(4, &nums4, is_arithmetic_progression(&nums4));
    print_result(5, &nums5, is_arithmetic_progression(&nums5));
}
