#include <algorithm>
#include <print>
#include <ranges>
#include <vector>

auto all_equal(auto range) -> bool {
    if (std::ranges::empty(range)) return false;
    auto f = *std::ranges::begin(range);
    return std::ranges::all_of(range, [f](auto val) { return val == f; });
}

auto is_arithmetic_progression(auto nums) -> bool {
    std::ranges::sort(nums);
    return all_equal(nums | std::views::adjacent_transform<2>(std::minus{}));
}

int main() {
    auto nums1 = std::vector{1, 3, 5, 7, 9};
    auto nums2 = std::vector{9, 1, 7, 5, 3};
    auto nums3 = std::vector{1, 2, 4, 8, 16};
    auto nums4 = std::vector{5, -1, 3, 1, -3};
    auto nums5 = std::vector{1.5, 3., 0., 4.5, 6.};

    std::println("Example 1:\nInput: {}\nOutput: {}\n", nums1, is_arithmetic_progression(nums1));
    std::println("Example 2:\nInput: {}\nOutput: {}\n", nums2, is_arithmetic_progression(nums2));
    std::println("Example 3:\nInput: {}\nOutput: {}\n", nums3, is_arithmetic_progression(nums3));
    std::println("Example 4:\nInput: {}\nOutput: {}\n", nums4, is_arithmetic_progression(nums4));
    std::println("Example 5:\nInput: {}\nOutput: {}\n", nums5, is_arithmetic_progression(nums5));

    return 0;
}
