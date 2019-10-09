use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
pub struct Solution {}
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::bst_helper(&nums[..])
    }

    fn bst_helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() { return None }
        Some(Rc::new(RefCell::new(TreeNode{
            val: nums[nums.len() / 2],
            left: Solution::bst_helper(&nums[0..(nums.len()/2)]),
            right: Solution::bst_helper(&nums[(nums.len()/2+1)..]),
        })))
    }
}


fn main() {
    let tree = Solution::sorted_array_to_bst(vec![-10,-3,-1, 0,5,9]);
    println!("tree = {:?}", tree);
}
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test_108() {
//        assert_eq!(Solution::sorted_array_to_bst(vec![-10,-3,0,5,9]), tree![0,-3,9,-10,null,5]);
//        assert_eq!(Solution::sorted_array_to_bst(vec![]), tree![]);
//    }
//}

//
//fn main() {
//    let arr = vec![1,2,3,4,5,6,7,8,9,10];
//    let mut result = Vec::new();
//    let (mut root, mut left, mut right) = tree(&arr);
//    result.push(root);
//    if left.len() != 0 && right.len() != 0 {
//        let (one, left2, right2) = tree(&left);
//
//    }
//
//    println!("Hello, world!");
//}
//
//fn tree(arr: &Vec<u64>) -> (u64, Vec<u64>, Vec<u64>) {
//    let length = arr.len();
//    let mut left = Vec::new();
//    let mut right = Vec::new();
//    let mut root_index = 0;
//    let mut root: u64 = 0;
//    while length > 0 {
//        if length % 2 == 0 {
//            root_index = length / 2 + 1;
//        }
//        else {
//            root_index = length / 2;
//        }
//        root = arr[root_index];
//        for i in 0..root_index {
//            left.push(arr[i]);
//        }
//        for j in (root_index + 1) ..length {
//            right.push(arr[j]);
//        }
//
//    }
//    (root, left, right)
//}