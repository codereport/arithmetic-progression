def is_arithmetic_progression(nums):
    nums.sort()
    return len(set(b - a for a, b in zip(nums, nums[1:]))) == 1


def main():
    nums1 = [1, 3, 5, 7, 9]
    nums2 = [9, 1, 7, 5, 3]
    nums3 = [1, 2, 4, 8, 16]
    nums4 = [5, -1, 3, 1, -3]
    nums5 = [1.5, 3.0, 0.0, 4.5, 6.0]

    print(f"Example 1:\nInput: {nums1}\nOutput: {is_arithmetic_progression(nums1)}\n")
    print(f"Example 2:\nInput: {nums2}\nOutput: {is_arithmetic_progression(nums2)}\n")
    print(f"Example 3:\nInput: {nums3}\nOutput: {is_arithmetic_progression(nums3)}\n")
    print(f"Example 4:\nInput: {nums4}\nOutput: {is_arithmetic_progression(nums4)}\n")
    print(f"Example 5:\nInput: {nums5}\nOutput: {is_arithmetic_progression(nums5)}\n")


if __name__ == "__main__":
    main()
