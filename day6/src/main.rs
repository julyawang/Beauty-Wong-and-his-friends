use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
//pub struct Solution {}
//impl Solution {
//    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
//        Solution::bst_helper(&nums[..])
//    }
//
//    fn bst_helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
//        if nums.is_empty() { return None }
//        Some(Rc::new(RefCell::new(TreeNode{
//            val: nums[nums.len() / 2],
//            left: Solution::bst_helper(&nums[0..(nums.len()/2)]),
//            right: Solution::bst_helper(&nums[(nums.len()/2+1)..]),
//        })))
//    }
//}


fn main() {
    let tree1 = tree(&vec![-10,-3,-1, 0,5,9]);
    let tree2 = tree(&vec![1,2,3,4,5,6,7,8,9,10]);
    println!("tree1 = {:?}", tree1);
    println!("tree2 = {:?}", tree2);
}



fn tree(arr: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    tree_helper(&arr[..])
}

fn tree_helper(arr: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if arr.is_empty() {
        return None
    }
    else {
        Some(Rc::new(RefCell::new(TreeNode{
            val: arr[arr.len() / 2],
            left: tree_helper(&arr[0..(arr.len()/2)]),
            right: tree_helper(&arr[(arr.len()/2+1)..]),
        })))

    }
}