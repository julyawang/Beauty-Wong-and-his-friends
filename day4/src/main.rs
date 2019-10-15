mod four;


fn main() {
//    let mut vec_array = Vec::new();
//    for i in 1..11 {
////        vec_array.push(i);
////    }
    let mut vec = vec![2,1,5,3,7,8,4,9,6];
    let sum = 7;

    //let (a, b) = four::find_elements_for_sum_1(&vec_array, sum);
    // let (c, d) = four::find_elements_for_sum_2(&vec_array, sum);
    let (c, d) = four::find_elements_for_sum_3(vec, sum);



}
