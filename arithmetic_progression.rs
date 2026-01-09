// https://play.rust-lang.org/?version=nightly&mode=debug&edition=2024&gist=9729edf14743236bf3e936225c8d7880
#![feature(iter_map_windows)]

use itertools::Itertools;
use core::{fmt::Debug, ops::Sub, cmp::Ordering};

fn is_arithmetic_progression(
    nums: impl IntoIterator<Item: Copy + TotalOrd + Sub<Output: PartialEq>>,
) -> bool {
    nums.into_iter()
        .sorted_by(TotalOrd::total_cmp)
        .map_windows(|&[a, b]| b - a)
        .all_equal()
}

trait TotalOrd {
    fn total_cmp(&self, rhs: &Self) -> Ordering;
}

impl<T: TotalOrd> TotalOrd for &T {
    fn total_cmp(&self, rhs: &Self) -> Ordering {
        (**self).total_cmp(*rhs)
    }
}

macro_rules! impl_float {
    ($($ty:ty)+) => {
        $(
            impl TotalOrd for $ty {
                fn total_cmp(&self, rhs: &Self) -> Ordering {
                    <$ty>::total_cmp(self, rhs)
                }
            }
        )+
    }
}

impl_float!(f32 f64);

macro_rules! impl_int {
    ($($ty:ty)+) => {
        $(
            impl TotalOrd for $ty {
                fn total_cmp(&self, rhs: &Self) -> Ordering {
                    <$ty>::cmp(self, rhs)
                }
            }
        )+
    }
}

impl_int!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

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
