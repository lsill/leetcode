//
// Created by lsill on 2022/11/3.
//

#ifndef NEW_C_STUDY_STRTEST_H
#define NEW_C_STUDY_STRTEST_H

#include <string>
#include <unordered_map>

using namespace std;

class StringPra
{
private:
    bool expand(const string &s, const string &t);
public:
    int maxRepeating(string sequence, string word);
    bool halvesAreAlike(string s);
    int expressiveWords(string s, vector<string> &words);
    int minLength(string s);
};

#endif //NEW_C_STUDY_STRTEST_H
