impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {

        if n <= 0 {
            return false;
        }
        let log_base4 = (n as f64).log(4.0);
        log_base4 == log_base4.floor()
    }
}
