

pub fn sortedArrayToBST(int[] nums) {
    if(nums.length==0)
    return null;
    return BST(nums,0,nums.length-1);
}

pub fn BST(int[] nums,int lo,int hi) {
    if(lo>hi)
    return null;
    int mid = (hi-lo)/2+lo;
    TreeNode node = new TreeNode(nums[mid]);
    node.left = BST(nums,lo,mid-1);
    node.right = BST(nums,mid+1,hi);
    return node;
}

