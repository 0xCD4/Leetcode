impl Solution {
    pub fn reverse(x: i32) -> i32 {

       let negative = x < 0;

       let reversed_str: String = x.abs().to_string().chars().rev().collect(); // store the integer into a string
       let reversed = reversed_str.parse::<i32>().unwrap_or(0); // parse the string into i32

       if negative{ // check whether the integer is negative or positive
        -reversed
       }else{
        reversed
       }
