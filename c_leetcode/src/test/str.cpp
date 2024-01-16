
#include "strTest.h"
#include <vector>
#include <string>
#include <iostream>

int main()
{
    StringHard* ha = new StringHard;
    std::cout<< ha->count("1","12",1,8) << std::endl;
    delete ha;
}