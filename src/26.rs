impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let max = nums.len();
        let mut i = 1;
        let mut pivot = 0;
        while i < max {
            let last_num = nums[pivot];
            if nums[i] > last_num {
                pivot += 1;
                nums[pivot] = nums[i];
            }
            i += 1;
        }
        return (pivot + 1) as i32;
    }
}