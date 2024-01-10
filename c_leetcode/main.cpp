#include <iostream>
#include "str_pra/strtest1.h"
#include "design/degisn1.h"
#include "arr_pra/arr_ra.h"
#include "tree_pra/tree_pra.h"
#include "sdudy_pra/study_pra.h"
using namespace std;

int main() {
    cout << "Hello, World!" << endl;
    BusyStudent* stu = new(BusyStudent);
    vector<int> nums;
    nums.insert(3);
    nums.insert(4);
    nums.insert(5);
    nums.insert(2);
    stu->maxProduct();
    delete(stu);
}

//[[4],[1,"roxxb"],[4,"luiqm"],[2,"qutzx"],[3,"zkugh"]]