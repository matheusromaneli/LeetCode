impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        for i in 0..haystack.len(){
            if haystack[i..].starts_with(&needle){
                return i as i32;
            }
        }
        return -1;
    }
}