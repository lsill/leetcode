//
// Created by lsill on 2022/11/3.
//

#ifndef NEW_C_STUDY_STRTEST_H
#define NEW_C_STUDY_STRTEST_H

#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

class StringPra
{
private:
    bool expand(const string &s, const string &t);
public:
    int maxRepeating(string sequence, string word);
    bool halvesAreAlike(string s);
    int expressiveWords(string s, vector<string> &words);
    int countWords(vector<string>& words1, vector<string>& words2);
};

class StringSimple {
public:
    int maximumNumberOfStringPairsSelf(vector<string>& words);
    int maximumNumberOfStringPairs(vector<string>& words);
    vector<string> splitWordsBySeparatorSelf(vector<string>& words, char separator);
    vector<string> splitWordsBySeparator(vector<string>& words, char separator);
    string maximumOddBinaryNumberSelf(string s);
    string maximumOddBinaryNumber(string s);
};

class StringMid {
public:
    vector<vector<string>> groupAnagrams(vector<string>& strs);
    int minimumOperations(string num);
};

#endif //NEW_C_STUDY_STRTEST_H
