impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }
        let doubled_s = format!("{}{}", s, s);
        doubled_s.contains(&goal)
    }
}
