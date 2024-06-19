impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut roman_integer = HashMap::new();
        roman_integer.insert('I', 1);
        roman_integer.insert('V', 5);
        roman_integer.insert('X', 10);
        roman_integer.insert('L', 50);
        roman_integer.insert('C', 100);
        roman_integer.insert('D', 500);
        roman_integer.insert('M', 1000);

          let mut result = 0;
          let mut prev_value = 0;

        for c in s.chars().rev() {
            let value = roman_integer.get(&c).unwrap();
            if *value < prev_value {
                result -= value;
            } else {
                result += value;
            }
            prev_value = *value;
        }

        result
    }
}
