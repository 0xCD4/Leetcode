impl Solution {
    pub fn to_hex(num: i32) -> String {
        // Special case for zero
        if num == 0 {
            return "0".to_string();
        }

        // Two's complement mask for 32-bit integer
        let mut n = num as u32;
        let mut hex = String::new();
        let hex_chars = "0123456789abcdef".chars().collect::<Vec<_>>();

        // Convert the number to hexadecimal
        while n != 0 {
            let digit = (n & 0xF) as usize; // Extract the last 4 bits
            hex.push(hex_chars[digit]);
            n >>= 4; // Shift right by 4 bits
        }

        // Reverse the string as the least significant digit is at the end
        hex.chars().rev().collect()
    }
}
