//
// Created by lsill on 2024/2/10.
//

#include "tree_base.h"
#include <vector>
#include <iostream>

int main() {
    TreeSimple sim;
    TreeNode node2 {3};
    TreeNode node1{2, &node2, nullptr};
    TreeNode node0{1, nullptr, &node1};

    auto ans = sim.inorderTraversal(&node0);
    for (auto& a : ans) {
        std::cout << a <<" ";
    }
}
