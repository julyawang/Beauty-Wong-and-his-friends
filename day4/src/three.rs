
pub fn find_elements_for_sum(arr: Vec<i32>, sum: i32) -> (i32, i32) {
    for ele in arr {
        if arr.contains(&(sum - ele)) {
            return (ele, sum - ele);
        }
    }
    (_, _)
}