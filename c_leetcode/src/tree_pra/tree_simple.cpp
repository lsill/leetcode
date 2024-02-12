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

// 145. 二叉树的后序遍历
//给你一棵二叉树的根节点 root ，返回其节点值的 后序遍历 。
//示例 1：
//输入：root = [1,null,2,3]
//输出：[3,2,1]
//示例 2：
//输入：root = []
//输出：[]
//示例 3：
//输入：root = [1]
//输出：[1]
//提示：
//树中节点的数目在范围 [0, 100] 内
//-100 <= Node.val <= 100
//进阶：递归算法很简单，你可以通过迭代算法完成吗？
// 迭代算法
std::vector<int> TreeSimple::postorderTraversal(TreeNode *root) {
    std::vector<int> ans;
    std::stack<TreeNode*> stk;
    if (root == nullptr) {
        return ans;
    }
    TreeNode* prev = nullptr;
    while (root != nullptr || !stk.empty()) {
        while (root != nullptr) {
            stk.emplace(root);
            root = root->left;
        }
        root = stk.top();
        stk.pop();
        if (root->right == nullptr || root->right == prev) {
            ans.emplace_back(root->val);
            prev = root;
            root = nullptr;
        } else {
            stk.emplace(root);
            root = root->right;
        }
    }
    return ans;
}

// 144. 二叉树的前序遍历
//已解答
//简单
//相关标签
//相关企业
//给你二叉树的根节点 root ，返回它节点值的 前序 遍历。
//示例 1：
//输入：root = [1,null,2,3]
//输出：[1,2,3]
//示例 2：
//输入：root = []
//输出：[]
//示例 3：
//输入：root = [1]
//输出：[1]
//示例 4：
//输入：root = [1,2]
//输出：[1,2]
//示例 5：
//输入：root = [1,null,2]
//输出：[1,2]
//提示：
//树中节点数目在范围 [0, 100] 内
//-100 <= Node.val <= 100
//进阶：递归算法很简单，你可以通过迭代算法完成吗？
// 迭代算法
std::vector<int> preorderTraversal(TreeNode *root) {
    std::vector<int> res;
    if (root == nullptr) {
        return res;
    }

    std::stack<TreeNode*> stk;
    TreeNode* node = root;
    while (!stk.empty() || node != nullptr) {
        while (node != nullptr) {
            res.emplace_back(node->val);
            stk.emplace(node);
            node = node->left;
        }
        node = stk.top();
        stk.pop();
        node = node->right;
    }
    return res;
}