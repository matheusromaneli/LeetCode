use std::collections::HashMap;
use std::cmp::max;

impl Solution {

    pub fn maximum_total_damage(power: Vec<i32>) -> i64 {
        let mut scores:HashMap<i32,i64> = HashMap::new();
        let mut max_spell = 0;
        let mut min_spell = i32::MAX;
        for &value in &power {
            match scores.get_mut(&value){
                Some(mut score) => {
                    (*score) += value as i64;
                },
                None => {
                    scores.insert(value,value as i64);
                },
            }
            if value > max_spell {
                max_spell = value;
            }
            if value < min_spell{
                min_spell = value;
            }
        }

        let mut cache = HashMap::new();
        for spell in (min_spell..=max_spell).rev(){
            let not_cast = match cache.get(&(spell+1)){
                Some(&cached) => cached,
                None => 0,
            };
            let cast = match scores.get(&spell) {
                Some(&score) => {
                    let future = match cache.get(&(spell+3)){
                        Some(&cached) => cached,
                        None => 0,
                    };
                    score as i64 + future
                },
                None => 0,
            };
            cache.insert(spell, max(cast, not_cast));
        }
        match cache.get(&min_spell){
            Some(&cached) => cached,
            None => 0,
        }
    }
}