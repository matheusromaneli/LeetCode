use std::collections::HashMap;
use std::cmp::max;

impl Solution {

    pub fn choose_index(index: usize, point: i32, points: &Vec<i32>) -> usize{
        let mut next = index;
        let mut next_value = (*points).get(next).copied().unwrap_or(i32::MAX);
        while (next_value <= point + 1) {
            next += 1;
            next_value = (*points).get(next).copied().unwrap_or(i32::MAX);
        } 
        return next;
    }

    pub fn choose_delete(index: usize, points: &Vec<i32>, points_amount: &HashMap<i32,i32>, cache: &mut HashMap<usize, i32>) -> i32 {
        if index >= (*points).len() {
            return 0;
        }
        match (*cache).get(&index) {
            Some(&cached) => cached,
            None => {
                let current_point = match (*points).get(index){
                    Some(&point) => point,
                    None => 0,
                };
                let current_amount = match (*points_amount).get(&current_point){
                    Some(&amount) => amount,
                    None => 0,
                };
                let next = Self::choose_index(index, current_point, points);
                // println!("Next if choose {}: {} else {}", current_point, next, index+1);
                let pass = Self::choose_delete(index+1, points, points_amount, cache);
                let current_total = current_point * current_amount;
                if next == 0 {
                    // println!("Delete {}: Yes({}) No({})", current_point, current_total, pass);
                    let result = max(pass,current_total);
                    (*cache).insert(index, result);
                    return result;
                }
                let delete = Self::choose_delete(next, points, points_amount, cache) + current_total;
                // println!("Delete {}: Yes({}) No({})", current_point, delete, pass);
                let result = max(pass, delete);
                (*cache).insert(index, result);
                return result;
            }
        }        
    }

    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut vals: HashMap<i32, i32> = HashMap::new();
        for &num in &nums{
            match vals.get_mut(&num){
                Some(count) => {(*count) += 1;},
                None => {vals.insert(num, 1);},
            };
        }
        let mut points: Vec<i32> = vals.keys().copied().collect();
        points.sort();
        let mut cache: HashMap<usize, i32> = HashMap::new();
        Self::choose_delete(0, &points, &vals, &mut cache)
    }
}