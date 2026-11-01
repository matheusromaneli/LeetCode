pub struct Solution {
}


impl Solution {

    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let mut iterations: i32 = 0;
        let min = nums.iter().min().unwrap();
        for i in 0..nums.len() {
            iterations += nums[i] - min;
        }
        iterations
    }
}
