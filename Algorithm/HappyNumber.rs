impl Solution {

    pub fn is_square(mut n: i32) -> i32{

        let mut num = 0;
        while n >0 {
            let digit = n%10;
            num+= digit * digit;
            n/=10
        }
        num


    }

    pub fn is_happy(mut n: i32) -> bool {
    let mut seen = std::collections::HashSet::new();

    while n != 1 && !seen.contains(&n) {
        seen.insert(n);
        n = Self::is_square(n);
    }

    n == 1


    }
}
