class Solution:
    def sortedArrayToBST(self, nums):
        """
        :type nums: List[int]
        :rtype: TreeNode
        """
        if not nums:
            return None
        else:
            mid=len(nums)//2
            tn=TreeNode(nums[mid])
            nums1=nums[0:mid]
            nums2=nums[mid+1:len(nums)]
            tn.left=self.sortedArrayToBST(nums1)
            tn.right=self.sortedArrayToBST(nums2)
        return tn

def main():
    """[main]
    """
    array = [0, 1, 2, 3, 4, 5]
    meili = Solution()
    meili_array = meili.sortedArrayToBST(array)
    print(meili_array)


if __name__ == '__main__':
    main()