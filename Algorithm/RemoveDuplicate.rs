impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut last_unique_index = 0;

        for i in 1..nums.len() {
            if nums[i] != nums[last_unique_index] {
                last_unique_index += 1;
                nums[last_unique_index] = nums[i];
            }
        }

        (last_unique_index + 1) as i32
    }
}
