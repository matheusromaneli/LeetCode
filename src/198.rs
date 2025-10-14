use std::collections::HashMap;
use std::cmp::max;

impl Solution {

    pub fn choose_rob(current: i32, nums: &Vec<i32>, cache: &mut HashMap<i32,i32>) -> i32{
        if current >= nums.len() as i32{
            return 0;
        }
        match (*cache).get(&current){
            Some(&cached) => cached,
            None => {
                let house_money: i32 = match nums.get(current as usize) {
                    Some(&x) => x,
                    None => 0,
                };
                let next_house = Self::choose_rob(current+1, nums, cache);
                let robber = Self::choose_rob(current+2, nums, cache) + house_money;
                // println!("Rob house {}: Yes({}) No ({})", current, robber, next_house);
                let result = max(robber, next_house);
                (*cache).insert(current, result);
                result
            }
        }
    }

    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut cache: HashMap<i32,i32> = HashMap::new();
        Self::choose_rob(0, &nums, &mut cache)
    }
}