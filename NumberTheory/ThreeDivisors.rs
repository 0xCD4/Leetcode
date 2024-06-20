impl Solution {
    pub fn is_three(n: i32) -> bool {
        if n < 4 {
            return false; // The smallest number with exactly three divisors is 4 (2^2).
        }
        
        let sqrt_n = (n as f64).sqrt() as i32;
        
        if sqrt_n * sqrt_n != n {
            return false; // n is not a perfect square.
        }
        
        // Check if sqrt_n is a prime number
        if sqrt_n <= 1 {
            return false; // 1 is not a prime number.
        }
        if sqrt_n <= 3 {
            return true; // 2 and 3 are prime numbers.
        }
        if sqrt_n % 2 == 0 || sqrt_n % 3 == 0 {
            return false; // Exclude multiples of 2 and 3.
        }
        
        let mut i = 5;
        while i * i <= sqrt_n {
            if sqrt_n % i == 0 || sqrt_n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        
        true // sqrt_n is a prime number.
    }
}
