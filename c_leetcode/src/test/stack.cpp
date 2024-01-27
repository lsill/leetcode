//
// Created by lsill on 2024/1/11.
//

#include "stack_base.h"

int main()
{
    StringStack *st = new StringStack;
    st->minLength("ABAB");
    delete st;
}