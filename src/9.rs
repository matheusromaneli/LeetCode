impl Solution {
    pub fn is_palindrome_str(x: String) -> bool{
        let size = x.len();
        let middle = size /2;
        for i in 0..middle {
            let mirror = size - 1 - i;
            if x[i..i+1] != x[mirror..mirror+1]{
                return false;
            }
        }
        return true;
    }

    pub fn is_palindrome(x: i32) -> bool {
        let num_str = x.to_string();
        return Self::is_palindrome_str(num_str);
    }
}