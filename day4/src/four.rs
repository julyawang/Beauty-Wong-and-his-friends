
// T(n^2)
pub fn find_elements_for_sum_1(arr: &Vec<i32>, sum: i32) -> (i32, i32) {
    for ele in arr {
        if arr.contains(&(sum - ele)) {
            println! ("a = {}, b = {}", ele.clone(), (sum - ele).clone());
            continue;
        }
    }
    (0, 0)
}


pub fn find_elements_for_sum_2(arr: &Vec<i32>, sum: i32) -> (i32, i32) {
    for i in 0..10 {
        for j in i..10 {
            if arr[i] == sum - arr[j] {
                println! ("a = {}, b = {}", arr[i].clone(), arr[j].clone());
                continue;
            }
        }
    }
    (0, 0)
}

// T(n*log_n)
// 先快排，再查找：快排O(NlogN)，查找O(N)；所以总的时间复杂度为：O(NlogN)
pub fn find_elements_for_sum_3(mut arr: Vec<i32>, sum: i32) -> (i32, i32) {
    arr.sort_unstable();
    println!("arr: {:?}", arr);
    let mut i = 0;
    let mut j = arr.clone().len() - 1;
    while i < j {
        if arr[i] + arr[j]== sum {
            println! ("a = {}, b = {}", arr[i].clone(), arr[j].clone());
            i = i + 1;
        }
		else if arr[i] + arr[j] < sum {
            i = i + 1;
        }
		else {
            j = j - 1;
        }
    }
    (0 as i32, 0 as i32)

}