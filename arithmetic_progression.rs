// https://play.rust-lang.org/?version=nightly&mode=debug&edition=2024&gist=9729edf14743236bf3e936225c8d7880

use itertools::Itertools;
use std::{fmt::Debug, ops::Sub};

fn is_arithmetic_progression(
    nums: impl IntoIterator<Item: Copy + PartialOrd + Sub<Output: PartialEq>>,
) -> bool {
    nums.into_iter()
        .sorted_by(|a, b| a.partial_cmp(b).unwrap())
        .map_windows(|&[a, b]| b - a)
        .all_equal()
}

fn print_result(example: usize, nums: impl Debug, result: bool) {
    print!("Example {example}:\nInput: {nums:?}\nOutput: {result}\n");
}

fn main() {
    let nums1 = [1, 3, 5, 7, 9];
    let nums2 = [9, 1, 7, 5, 3];
    let nums3 = [1, 2, 4, 8, 16];
    let nums4 = [5, -1, 3, 1, -3];
    let nums5 = [1.5, 3.0, 0.0, 4.5, 6.0];

    print_result(1, &nums1, is_arithmetic_progression(&nums1));
    print_result(2, &nums2, is_arithmetic_progression(&nums2));
    print_result(3, &nums3, is_arithmetic_progression(&nums3));
    print_result(4, &nums4, is_arithmetic_progression(&nums4));
    print_result(5, &nums5, is_arithmetic_progression(&nums5));
}
