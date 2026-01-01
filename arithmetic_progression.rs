// https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=e57e29687a339003a20cbee8e120ca3c

use itertools::Itertools;
use std::{fmt::Display, ops::Sub};

fn is_arithmetic_progression(
    nums: impl IntoIterator<Item: Copy + PartialOrd + Sub<Output: PartialEq>>,
) -> bool {
    nums.into_iter()
        .sorted_by(|a, b| a.partial_cmp(b).unwrap())
        // Nightly Rust:
        // .map_windows(|&[a, b]| b - a)
        .tuple_windows()
        .map(|(a, b)| b - a)
        .all_equal()
}

fn print_result(example: usize, nums: &[impl Display], result: bool) {
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
