//
// Created by lsill on 2024/2/10.
//

#include "tree_base.h"
#include <stack>

// 94. 二叉树的中序遍历
//给定一个二叉树的根节点 root ，返回 它的 中序 遍历 。
//示例 1：
//输入：root = [1,null,2,3]
//输出：[1,3,2]
//示例 2：
//输入：root = []
//输出：[]
//示例 3：
//输入：root = [1]
//输出：[1]
//提示：
//树中节点数目在范围 [0, 100] 内
//-100 <= Node.val <= 100
// 栈迭代算法
std::vector<int> TreeSimple::inorderTraversal_iter(TreeNode *root) {
    std::vector<int> res;
    std::stack<TreeNode*> stk;
    while (root != nullptr || !stk.empty()) {
        while (root != nullptr) {
            stk.push(root);
            root = root->left;
        }
        root = stk.top();
        stk.pop();
        res.push_back(root->val);
        root = root->right;
    }
    return res;
}

std::vector<int> TreeSimple::inorderTraversal(TreeNode *root) {
    std::vector<int> ans;
    inorderTraversal_dfs(ans, root);
    return ans;
}

void TreeSimple::inorderTraversal_dfs(std::vector<int> &ans, TreeNode *node) {
    if (node == nullptr) {
        return;
    }
    if (node->left) {
        inorderTraversal_dfs(ans, node->left);
    }
    ans.push_back(node->val);
    if (node->right) {
        inorderTraversal_dfs(ans, node->right);
    }
}