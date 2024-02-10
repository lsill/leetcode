//
// Created by lsill on 2024/2/10.
//

#ifndef LC_TREE_BASE_H
#define LC_TREE_BASE_H

#include <vector>

struct TreeNode {int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};


class TreeSimple {
public:
    std::vector<int> inorderTraversal(TreeNode* root);
    std::vector<int> inorderTraversal_iter(TreeNode* root);
private:
    void inorderTraversal_dfs(std::vector<int>& ans, TreeNode* node);
};

#endif //LC_TREE_BASE_H
