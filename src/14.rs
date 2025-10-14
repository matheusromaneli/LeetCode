impl Solution {
    pub fn can_add(current: char, index: usize, strs: &Vec<String>) -> bool{
        for j in 0..strs.len(){
            match strs[j].chars().nth(index){
                Some(comparision) =>{
                    if current != comparision {
                        return false;
                    }
                },
                None => return false,
            }
        }
        return true;
    }
    
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut result = String::new();
        for i in 0.. strs[0].len() {
            match strs[0].chars().nth(i){
                Some(current) =>{
                    if Self::can_add(current, i, &strs){
                        result.push(current);
                    }
                    else{
                        return result;
                    }
                },
                None => return result,
            }
        }
        return result;
        
    }
}