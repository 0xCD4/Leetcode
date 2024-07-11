impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len(){
            return false
        }


        let mut count_s = [0;256];
        let mut count_t = [0;256];

        for(sc, tc) in s.chars().zip(t.chars()){
            count_s[sc as usize] += 1;
            count_t[tc as usize] += 1;
        }


        count_s == count_t





    }
}
