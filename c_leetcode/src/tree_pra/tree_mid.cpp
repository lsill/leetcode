//
// Created by lsill on 2024/2/21.
//

#include "tree_base.h"

#include <unordered_map>
#include <functional>

// 106. 从中序与后序遍历序列构造二叉树
//给定两个整数数组 inorder 和 postorder ，其中 inorder 是二叉树的
// 中序遍历， postorder 是同一棵树的后序遍历，请你构造并返回这颗 二叉树 。
//示例 1:
//输入：inorder = [9,3,15,20,7], postorder = [9,15,7,20,3]
//输出：[3,9,20,null,null,15,7]
//示例 2:
//输入：inorder = [-1], postorder = [-1]
//输出：[-1]
//提示:
//1 <= inorder.length <= 3000
//postorder.length == inorder.length
//-3000 <= inorder[i], postorder[i] <= 3000
//inorder 和 postorder 都由 不同 的值组成
//postorder 中每一个值都在 inorder 中
//inorder 保证是树的中序遍历
//postorder 保证是树的后序遍历
// 力扣比较好的解
TreeNode* TreeMid::buildTree(std::vector<int> &inorder, std::vector<int> &postorder) {
    int n = inorder.size();
    std::unordered_map<int, int> index;
    for (int i = 0; i < n; i++) {
        index[inorder[i]] = i;
    }

    std::function<TreeNode*(int, int, int, int)> dfs = [&](int in_l, int in_r, int post_l, int post_r) -> TreeNode* {
        if (post_l == post_r) { // 空节点
            return nullptr;
        }
        int left_size = index[postorder[post_r - 1]] - in_l; // 左子树的大小
        TreeNode *left = dfs(in_l, in_l + left_size, post_l, post_l + left_size);
        TreeNode *right = dfs(in_l + left_size + 1, in_r, post_l + left_size, post_r - 1);
        return new TreeNode(postorder[post_r - 1], left, right);
    };
    return dfs(0, n, 0, n); // 左闭右开区间
}