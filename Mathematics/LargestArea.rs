impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut max_area = 0.0;

        for i in 0..points.len() {
            for j in i+1..points.len() {
                for k in j+1..points.len() {
                    let area = Solution::triangle_area(&points[i], &points[j], &points[k]);
                    if area > max_area {
                        max_area = area;
                    }
                }
            }
        }

        max_area
    }

    fn triangle_area(p1: &Vec<i32>, p2: &Vec<i32>, p3: &Vec<i32>) -> f64 {
        let area = ((p1[0] * (p2[1] - p3[1]) +
                     p2[0] * (p3[1] - p1[1]) +
                     p3[0] * (p1[1] - p2[1])) as f64).abs() / 2.0;
        area
    }
}
