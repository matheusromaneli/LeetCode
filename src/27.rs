impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut pivot = nums.len() - 1;
        for i in (0..nums.len()).rev(){
            if nums[i] == val {
                nums[i] = nums[pivot];
                pivot -= 1;
            }
        }
        (pivot + 1) as i32
    }
}