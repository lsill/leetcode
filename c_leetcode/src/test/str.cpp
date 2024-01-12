
#include "strTest.h"
#include <vector>
#include <string>
#include <iostream>

int main()
{
    StringPra* pra = new StringPra;
    vector<string> word1 = {"leetcode","is","amazing","as","is"};
    vector<string> word2 =  {"amazing","leetcode","is"};
    std::cout << pra->countWords(word1,word2) << std::endl;
    delete pra;
}