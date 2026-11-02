use std::println;

pub struct Solution;

impl Solution {

    fn find_median(nums: &Vec<i32>) -> f32 {
        let mut nums = nums.clone();
        nums.sort();
        let mid = nums.len() / 2;
        let median: f32 = if nums.len() % 2 == 0 {
            (nums[mid - 1] + nums[mid]) as f32 / 2.0
        } else {
            nums[mid] as f32
        };
        median
    }

    fn choose_mean(nums: &Vec<i32>) -> i32 {
        // let mean = Self::find_mean(nums);
        let mean = Self::find_median(nums);
        let mut min = f32::MAX;
        for num in nums.iter() {
            min = min.min((*num as f32 - mean).abs());
        }
        let element = (mean - min) as i32;
        element
    }

    pub fn min_moves2(nums: Vec<i32>) -> i32 {
        let mean = Self::choose_mean(&nums);
        let mut moves = 0;
        for num in nums.iter() {
            moves += (num - mean).abs();
        }
        moves
    }
}