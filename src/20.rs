impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        let close = vec![')',']','}'];
        for value in s.chars() {
            if close.contains(&value){
                match stack.pop() {
                    Some(top) => {
                        match value {
                            ')' => if top != '(' {return false},
                            ']' => if top != '[' {return false},
                            '}' => if top != '{' {return false},
                            _ => {return false}
                        }
                    }
                    None => return false,
                }
            }
            else {
                stack.push(value);
            }
        }
        if stack.is_empty(){
            return true;
        }
        else {
            return false;
        }
    }
}