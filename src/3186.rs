use std::collections::HashMap;
use std::cmp::max;

impl Solution {

    pub fn next_cast(index: usize, spells: &Vec<i32>) -> usize{
        let mut next = index + 1;
        while (next < spells.len() && (spells[next] <= spells[index] + 2)){
            next += 1;
        }
        return next;
    }

    pub fn choose_spell(index: usize, spells: &Vec<i32>, amount: &HashMap<i32,i32>, cache: &mut Vec<i64>) -> i64 {
        if index >= spells.len() {
            return 0;
        }
        if cache[index] != -1{
            return cache[index];
        }
        let current_spell = spells[index];
        let current_damage =  current_spell as i64 * amount[&current_spell] as i64;

        let pass = Self::choose_spell(index + 1, spells, amount, cache);

        let next = Self::next_cast(index, spells);
        let cast = Self::choose_spell(next, spells, amount, cache) + current_damage;
        // println!("Cast {}: Yes({}) No({})", current_spell, cast, pass);
        let result = max(pass, cast);
        cache[index] = result;
        return result;
    }

    pub fn maximum_total_damage(power: Vec<i32>) -> i64 {
        let mut spell_amount: HashMap<i32, i32> = HashMap::new();
        for &spell in &power{
            if spell_amount.contains_key(&spell){
                spell_amount.insert(spell, spell_amount[&spell] + 1);
            }
            else {
                spell_amount.insert(spell, 1);
            }
        }

        let mut spell_damages: Vec<i32> = spell_amount.keys().copied().collect();
        spell_damages.sort();
        let mut cache: Vec<i64> = vec![-1; spell_damages.len()];
        Self::choose_spell(0, &spell_damages, &spell_amount, &mut cache)
    }
}