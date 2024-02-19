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
    std::vector<int> postorderTraversal(TreeNode* root);
    std::vector<int> preorderTraversal(TreeNode *root);
private:
    void inorderTraversal_dfs(std::vector<int>& ans, TreeNode* node);
};

class Node {
public:
    int val;
    std::vector<Node*> children;

    Node() {}

    Node(int _val) {
        val = _val;
    }

    Node(int _val, std::vector<Node*> _children) {
        val = _val;
        children = _children;
    }
};

class NodeSimple{
public:
    std::vector<int> postorder(Node* root);
    std::vector<int> postorderIter(Node* root);
};

#endif //LC_TREE_BASE_H
